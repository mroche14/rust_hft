use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::mem;
use crate::config::{BufferConfig, ColumnConfig};
use crate::prefetch::maybe_prefetch;
use crate::delta_encoding::delta_encode_column;

#[repr(C)]
pub struct SharedHeader {
    pub head: AtomicUsize,
    pub num_rows: usize,
    pub num_cols: usize,
}

pub struct CircularBuffer {
    header: Arc<SharedHeader>,
    data: Arc<[u8]>,
    capacity: usize,
    row_size: usize,
    column_info: Vec<ColumnConfig>,
    prefetch_enabled: bool,
    prefetch_distance: usize,
    delta_encoded_cols: Vec<usize>,
    last_values: Vec<f64>,
}

impl CircularBuffer {
    pub fn new(
        ptr: *mut u8,
        cfg: &BufferConfig,
        columns: Vec<ColumnConfig>,
        prefetch_enabled: bool,
        prefetch_distance: usize,
    ) -> Self {
        let header_ptr = ptr as *mut SharedHeader;
        let header = unsafe {
            let header_ref = &mut *header_ptr;
            header_ref.num_rows = cfg.num_rows;
            header_ref.num_cols = cfg.num_cols;
            header_ref.head.store(0, Ordering::SeqCst);
            Arc::new(SharedHeader {
                head: AtomicUsize::new(0),
                num_rows: header_ref.num_rows,
                num_cols: header_ref.num_cols,
            })
        };

        let row_size = cfg.num_cols * mem::size_of::<f64>();
        let data_ptr = unsafe { ptr.add(mem::size_of::<SharedHeader>()) };
        let data = unsafe { Arc::from_raw(std::slice::from_raw_parts(data_ptr, cfg.num_rows * row_size)) };

        let mut delta_encoded_cols = vec![];
        for (i, c) in columns.iter().enumerate() {
            if c.delta_encoding {
                delta_encoded_cols.push(i);
            }
        }

        let last_values = vec![0.0; cfg.num_cols];

        CircularBuffer {
            header,
            data,
            capacity: cfg.num_rows,
            row_size,
            column_info: columns,
            prefetch_enabled,
            prefetch_distance,
            delta_encoded_cols,
            last_values,
        }
    }

    #[inline]
    fn row_ptr(&self, idx: usize) -> *mut f64 {
        let base = self.data.as_ptr();
        unsafe { base.add((idx % self.capacity) * self.row_size) as *mut f64 }
    }

    pub fn write_row(&mut self, row: &[f64]) {
        let idx = self.header.head.fetch_add(1, Ordering::SeqCst);
        let row_ptr = self.row_ptr(idx);

        maybe_prefetch(row_ptr as *const u8, self.prefetch_distance, self.prefetch_enabled);

        unsafe {
            for (col_i, val) in row.iter().enumerate() {
                let encoded_val = if self.delta_encoded_cols.contains(&col_i) {
                    delta_encode_column(self.last_values[col_i], *val)
                } else {
                    *val
                };
                *row_ptr.add(col_i) = encoded_val;
            }
        }

        for (col_i, val) in row.iter().enumerate() {
            self.last_values[col_i] = *val;
        }
    }

    pub fn head(&self) -> usize {
        self.header.head.load(Ordering::SeqCst)
    }

    pub fn read_rows(&self, start: usize, end: usize) -> Vec<Vec<f64>> {
        let mut out = Vec::with_capacity(end - start);
        for i in start..end {
            let rp = self.row_ptr(i);
            maybe_prefetch(rp as *const u8, self.prefetch_distance, self.prefetch_enabled);
            unsafe {
                let mut row = Vec::with_capacity(self.column_info.len());
                for col_i in 0..self.column_info.len() {
                    row.push(*rp.add(col_i));
                }
                out.push(row);
            }
        }
        out
    }
}

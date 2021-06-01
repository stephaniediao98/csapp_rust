use std::alloc::{alloc, dealloc, Layout};

pub struct VecRec {
    pub len: i64,
    pub data: *mut i64,
}

impl VecRec {
    /// Create vector of specified length
    pub unsafe fn new(len: i64) -> Option<*mut VecRec> {
        let vec_layout = Layout::new::<VecRec>();
        let result = alloc(vec_layout) as *mut VecRec;
        let mut data = &mut 0_i64 as *mut i64;
        (*result).len = len;

        /* Allocate array */
        if len > 0 {
            if let Ok(layout) = Layout::from_size_align(8_usize*len as usize, 8) {
                data = alloc(layout) as *mut i64;
            } else {
                dealloc(result as *mut u8, vec_layout);
                return None
            }
        }
        (*result).data = data;
        Some(result.into())
    }

    /// Retrieve vector element and store at dest.
    /// Return 0 (out of bounds) or 1 (successful)
    pub unsafe fn get_vec_element(&self, index: i64, dest: *mut i64) -> usize {
        if index < 0 || index >= self.len {
            return 0;
        }
        *dest = *(self.data.offset(index as isize));
        1
    }

    /// Store val at vector element.
    /// Return 0 (out of bounds) or 1 (successful)
    pub unsafe fn set_vec_element(&self, index: i64, val: i64) -> usize {
        if index < 0 || index >= self.len {
            return 0;
        }
        *(self.data.offset(index as isize)) = val;
        1
    }

    /// Return length of vector
    pub unsafe fn vec_length(&self) -> i64 {
        self.len
    }

    /// Return the start of vector
    pub unsafe fn get_vec_start(&self) -> *mut i64 {
        self.data
    }
}


pub unsafe fn combine1(v: *mut VecRec, dest: *mut i64) {
    *dest = 0;
    for i in 0..(*v).vec_length(){
        let mut val: i64 = 0;
        (*v).get_vec_element(i, &mut val as *mut i64);
        *dest = *dest + val;
    }
}

pub unsafe fn combine2(v: *mut VecRec, dest: *mut i64) {
    *dest = 0;
    let length = (*v).vec_length();
    for i in 0..length {
        let mut val: i64 = 0;
        (*v).get_vec_element(i, &mut val as *mut i64);
        *dest = *dest + val;
    }
}

pub unsafe fn combine3(v: *mut VecRec, dest: *mut i64) {
    *dest = 0;
    let length = (*v).vec_length();
    let data = (*v).get_vec_start();
    for i in 0..length {
        *dest = *dest + *data.offset(i as isize);
    }
}

pub unsafe fn combine4(v: *mut VecRec, dest: *mut i64) {
    let length = (*v).vec_length();
    let data = (*v).get_vec_start();
    let mut acc = 0;
    for i in 0..length {
        acc = acc + *data.offset(i as isize);
    }
    *dest = acc;
}

/* 2 x 1 loop unrolling*/
pub unsafe fn combine5(v: *mut VecRec, dest: *mut i64) {
    let length = (*v).vec_length();
    let limit = length - 1;
    let data = (*v).get_vec_start();
    let mut acc = 0;
    
    /* Combine 2 elements at a time */
    let mut i = 0;
    while i < limit {
        acc = (acc + *data.offset(i as isize)) + *data.offset((i+1) as isize);
        i += 2;
    }
    /* Handle last element if need be */
    for _ in i..length {
        acc = acc + *data.offset(limit as isize);
    }
    *dest = acc;
}

/* 2 x 2 loop unrolling */
pub unsafe fn combine6(v: *mut VecRec, dest: *mut i64) {
    let length = (*v).vec_length();
    let limit = length - 1;
    let data = (*v).get_vec_start();
    let mut acc0 = 0;
    let mut acc1 = 0;

    /* Combine 2 elements at a time */
    let mut i = 0;
    while i < limit {
        acc0 = acc0 + *data.offset(i as isize);
        acc1 = acc1 + *data.offset((i+1) as isize);
        i += 2;
    }
    
    /* Handle last element if need be */
    for _ in i..length {
        acc0 += *data.offset(limit as isize);
    }
    *dest = acc0 + acc1;
}

/* 2 x 1a loop unrolling */
pub unsafe fn combine7(v: *mut VecRec, dest: *mut i64) {
    let length = (*v).vec_length();
    let limit = length - 1;
    let data = (*v).get_vec_start();
    let mut acc = 0;

    /* Combine 2 elements at a time */
    let mut i = 0;
    while i < limit {
        acc = acc + (*data.offset(i as isize) + *data.offset((i+1) as isize));
        i += 2;
    }
    
    /* Handle last element if need be */
    if i == limit {
        acc += *data.offset(limit as isize);
    }
    *dest = acc;
}

pub unsafe fn combine4b(v: *mut VecRec, dest: *mut i64) {
    let length = (*v).vec_length();
    let data = (*v).get_vec_start();
    let mut acc = 0;
    for i in 0..length {
        if i >= 0 && i < (*v).len {
            acc += *data.offset(i as isize);
        }
    }
    *dest = acc;
}

fn main() {}

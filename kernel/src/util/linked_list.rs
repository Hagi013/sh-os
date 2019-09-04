use alloc::string::{String, ToString};
use core::mem::replace;

struct LinkedListNode<T>
where
    T: Copy,
    T: PartialEq,
{
    prev: Option<*mut LinkedListNode<T>>,
    next: Option<*mut LinkedListNode<T>>,
    data: T,
}

impl<T> LinkedListNode<T>
where
    T: Copy,
    T: PartialEq,
{
    pub fn new(data: T) -> Self {
        LinkedListNode {
            prev: None,
            next: None,
            data,
        }
    }
}

pub struct LinkedList<T>
where
    T: Copy,
    T: PartialEq,
{
    head: Option<*mut LinkedListNode<T>>,
    tail: Option<*mut LinkedListNode<T>>,
    count: usize,
}

impl<T> LinkedList<T>
where
    T: Copy,
    T: PartialEq,
{
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            count: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    // addするときは一番最後に入れる
    pub fn add(&mut self, mut data: T) -> Result<(), String> {
        unsafe {
            let mut node: LinkedListNode<T> = LinkedListNode::new(data);
            let pointer: *mut LinkedListNode<T> = &mut node as *mut LinkedListNode<T>;
            if let Some(last_data) = self.tail {
                self.tail = Some(pointer);
                node.next = None;
                (*last_data).next = Some(pointer);
                node.prev = Some(last_data);
            } else { // 基本的にはtailにデータが存在するはずなので、初回の追加を想定している
                self.tail = Some(pointer);
                if self.count == 0 && self.head.is_none() { // 初回追加の場合headも存在しないので、headとしてもデータを挿入する
                    self.head = Some(pointer);
                    node.prev = None;
                    node.next = None;
                }
            }
            self.count += 1;
            return Ok(());
        }
    }

    pub fn remove(&mut self, mut data: T) -> Result<(), String> {
        if self.len() == 0 {
            return Err("LinkedList length is 0.".to_string());
        }
        self.count -= 1;
        // 残った最後の一つの要素だった場合
        if self.count == 0 {
            self.head = None;
            self.tail = None;
            return Ok(());
        }

        unsafe {
            let pointer: *mut LinkedListNode<T> = self.get_pointer_from_data(data).ok_or("data is not existing in LinkedList.".to_string())?;

            if self.head.is_some() && self.tail.is_some() { // 基本的に要素が存在する場合は、headとtailは存在するはず
                let head: *mut LinkedListNode<T> = self.head.unwrap();
                let tail: *mut LinkedListNode<T> = self.tail.unwrap();

                if pointer != head && pointer != tail { // headとtailの要素が今回の削除対象ではない場合
                    if let Some(mut prev) = (*pointer).prev {
                        // prevのnextを更新
                        // (*prev).next = (*pointer).next;
                        replace(&mut (*prev).next, replace(&mut (*pointer).next, None));
                    }
                    if let Some(mut next) = (*pointer).next {
                        replace(&mut (*next).prev, replace(&mut (*pointer).prev, None));
                    }
                    // 後処理(これは必要なのか？)
//                    replace(&mut (*pointer).prev, None);
//                    replace(&mut (*pointer).next, None);
                    (*pointer).prev = None;
                    (*pointer).next = None;
                } else if pointer == head { // headが削除対象の場合
                    // *self.head = (*pointer).next;
                    replace(&mut self.head, replace(&mut (*pointer).next, None));
                    replace(&mut (*self.head.unwrap()).prev, None);
                } else if pointer == tail { // tailが削除対象の場合
                    replace(&mut self.tail, replace(&mut (*pointer).prev, None));
                    replace(&mut (*self.tail.unwrap()).next, None);
                } else {
                    self.head = None;
                    self.tail = None;
                }
                return Ok(());
            } else {
                return Err("Element in LinkedList is null.".to_string());
            }
            // ここに来ることはない
            return Ok(());
        }
    }

    pub fn change_order(&mut self, data: T, idx: usize) -> Result<(), String> {
        let src_node_ptr: *mut LinkedListNode<T> = self.get_pointer_from_data(data).ok_or("data is not existing in LinkedList".to_string())?;
        let order: usize =
            if idx >= self.len() { self.len() - 1 }
            else if idx < 0 { 0 }
            else { idx };

        unsafe {
            // はじめに対象のNodeの前後の紐付きを更新する
            if let Some(mut prev) = (*src_node_ptr).prev {
                replace(&mut (*prev).next, (*src_node_ptr).next);
            }
            if let Some(mut next) = (*src_node_ptr).next {
                replace(&mut (*next).prev, (*src_node_ptr).prev);
            }
            if order == 0 { // headになる
                if let Some(head_node_ptr) = self.head {
                    (*head_node_ptr).prev = Some(src_node_ptr);
                    (*src_node_ptr).next = Some(head_node_ptr);
                    (*src_node_ptr).prev = None;
                    return Ok(());
                }
                // もしheadが存在しない場合は、追加する(エラーとして終了でも良いが、、)
                return self.add(data);
            } else if order == self.len() - 1 {
                let tail_node_ptr: *mut LinkedListNode<T> = self.tail.ok_or("LinkedList is broken.".to_string())?;
                (*tail_node_ptr).next = Some(src_node_ptr);
                (*src_node_ptr).prev = Some(tail_node_ptr);
                (*src_node_ptr).next = None;
                return Ok(());
            } else {
                // 挿入したい場所における入れ替え操作
                let mut dest_order_node_ptr: *mut LinkedListNode<T> = self.get_pointer_from_index(order)
                    .ok_or("idx argument to change_order may be out of bound.".to_string())?;

                (*dest_order_node_ptr).prev = Some(src_node_ptr);
                (*src_node_ptr).next = Some(dest_order_node_ptr);

                let dest_order_prev_node_ptr = (*dest_order_node_ptr).prev
                    .ok_or("change_order target order node's state is broken in LinkedList.".to_string())?;
                (*dest_order_prev_node_ptr).next = Some(src_node_ptr);
                (*src_node_ptr).prev = Some(dest_order_prev_node_ptr);
                return Ok(());
            }
        }
        return Ok(());
    }

    pub fn get_position_from_data(&self, data: T) -> Option<usize> {
        if self.len() == 0 { return None; }
        let mut idx: usize = 0;
        let mut node: *mut LinkedListNode<T> = self.head.or(None)?;
        loop {
            unsafe {
                if data == (*node).data {
                    return Some(idx);
                }
                if let Some(next) = (*node).next {
                    idx += 1;
                    node = next;
                } else {
                    break;
                }
            }
        }
        return None;
    }

    pub fn get_data_from_position(&self, idx: usize) -> Option<T> {
        if idx <= 0 || idx >= self.count {
            return None;
        }
        let data_ptr: *mut LinkedListNode<T> = self.get_pointer_from_index(idx).or(None)?;
        return unsafe { Some((*data_ptr).data) };
    }

    pub fn get_next_data(&self, data: T) -> Option<T> {
        match self.get_position_from_data(data) {
            Some(idx) => {
                if idx + 1 >= self.len() {
                    return None;
                }
                let next_data_ptr: *mut LinkedListNode<T> = self.get_pointer_from_index(idx + 1).or(None)?;
                return unsafe { Some((*next_data_ptr).data) };
            },
            None => None,
        }
    }

    pub fn get_prev_data(&self, data: T) -> Option<T> {
        match self.get_position_from_data(data) {
            Some(idx) => {
                if idx <= 0 { return None; }
                let prev_data_ptr: *mut LinkedListNode<T> = self.get_pointer_from_index(idx - 1).or(None)?;
                return unsafe { Some((*prev_data_ptr).data) };
            },
            None => None,
        }
    }

    fn get_pointer_from_data(&self, data: T) -> Option<*mut LinkedListNode<T>> {
        if self.len() == 0 { return None }
        let mut node: *mut LinkedListNode<T>  = self.head.or(None)?;
        loop {
            unsafe {
                if data == (*node).data {
                    return Some(node);
                }
                if let Some(next) = (*node).next {
                    node = next;
                } else {
                    break;
                }
            }
        }
        return None;
    }

    fn get_pointer_from_index(&self, idx: usize) -> Option<*mut LinkedListNode<T>> {
        if idx < 0 || idx >= self.len() { return None; }
        let mut node: *mut LinkedListNode<T> = self.head.or(None)?;

        for i in 0..idx {
            unsafe {
                if let Some(next) = (*node).next {
                    node = next;
                } else {
                    return None;
                }
            }
        }
        return Some(node)
    }

}
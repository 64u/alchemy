#![allow(dead_code)]

/* TODO:
 * - Get the real start/end of memory.
 * - Properly reserve a section for the bitmap.
 */

/// Block size to use. Default is 4KB.
const BLOCK_SIZE: usize = 4000;
/// Memory offset to start paging at. Default is 2MB.
const OFFSET: usize = (1000 * 1000) * 2;
/// End of memory (hardcoded 128MB for now).
const END: usize = (1000 * 1000) * 128; 
/// Number of blocks.
const BLOCKS: usize = (END - OFFSET) / BLOCK_SIZE;

/// Represents a single page of memory in our map.
#[derive(Debug)]
pub enum Page {
  Free,
  Used
}

/// Some id <-> pointer transformative functions.
impl Page {
  /// Retrieve a pointer address from a block ID.
  fn as_ptr(id: usize) -> *mut [u8; BLOCK_SIZE] {
    (OFFSET + (id * BLOCK_SIZE)) as *mut _
  }

  /// Retrieve a block map ID from a pointer.
  fn as_id(ptr: *mut [u8; BLOCK_SIZE]) -> usize {
    (ptr as usize - OFFSET) / BLOCK_SIZE
  }
}

pub struct Manager {
  pages: *mut [Page; BLOCKS]
}

impl Manager {
  pub fn new() -> Manager {
    Manager { pages: OFFSET as *mut _ }
  }

  /**
   * Find and return the index of the first free block.
   * or return None if one doesn't exist.
   */
  fn find_free(&mut self) -> Option<usize> {
    for block in 0..BLOCKS {
      if let Page::Free = self.pages()[block] {
        return Some(block)
      }
    }

    return None
  }

  /// Allows mutable access to the internal block map.
  fn pages(&mut self) -> &mut [Page; BLOCKS] {
    unsafe { self.pages.as_mut().unwrap() }
  }

  /** 
   * Returns a mutable pointer to the first free block.
   * or None if one doesn't exist.
   */
  pub fn page(&mut self) -> Option<*mut [u8; BLOCK_SIZE]> {
    let index = self.find_free();
    match index {
      Some(index) => {
        // mark the block as used then return it
        self.pages()[index] = Page::Used;
        Some(Page::as_ptr(index))
      },

      None => None
    }
  }

  /// Frees the block at a given pointer.
  pub fn free(&mut self, ptr: *mut [u8; BLOCK_SIZE]) {
    let id = Page::as_id(ptr);
    self.pages()[id] = Page::Free;
  }

  /// "Probe" the block at `ptr` returning its freedom status.
  pub fn probe(&mut self, ptr: *mut [u8; BLOCK_SIZE]) -> &Page {
    &self.pages()[Page::as_id(ptr)]
  }
}

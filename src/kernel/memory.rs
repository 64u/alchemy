#![allow(dead_code)]

/* TODO:
 * - Allow freeing blocks.
 * - Mark blocks as used.
 * - Document.
 * - Get the real start/end of memory.
 * - Properly reserve a section for the bitmap.
 */

// Block size to use. Default is 4KB.
const BLOCK_SIZE: usize = 4000;
// Memory offset to start paging at. Default is 2MB.
const OFFSET: usize = (1000 * 1000) * 2;
// end of memory (hardcoded 128MB for now)
const END: usize = (1000 * 1000) * 128; 
// number of blocks
const BLOCKS: usize = (END - OFFSET) / BLOCK_SIZE;

// Represents a single page of memory.

enum Page {
  Free,
  Used
}

impl Page {
  fn as_ptr(id: usize) -> *mut [u8; BLOCK_SIZE] {
    (OFFSET + (id * BLOCK_SIZE)) as *mut _
  }
}

pub struct Manager {
  pages: *mut [Page; BLOCKS]
}

impl Manager {
  pub fn new() -> Manager {
    Manager { pages: OFFSET as *mut _ }
  }

  fn find_free(&mut self) -> Option<usize> {
    for block in 0..BLOCKS {
      if let Page::Free = self.pages()[block] {
        return Some(block)
      }
    }

    return None
  }

  fn pages(&mut self) -> &mut [Page; BLOCKS] {
    unsafe { self.pages.as_mut().unwrap() }
  }

  pub fn page(&mut self) -> Option<*mut [u8; BLOCK_SIZE]> {
    let index = self.find_free();
    match index {
      Some(index) => {
        Some(Page::as_ptr(index))
      },

      None => None
    }
  }
}

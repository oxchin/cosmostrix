// Copyright (c) 2025 rezk_nightky

use crate::cell::Cell;

#[derive(Clone, Debug)]
pub struct Frame {
    pub width: u16,
    pub height: u16,
    pub cells: Vec<Cell>,
    dirty_all: bool,
    dirty_map: Vec<bool>,
    dirty: Vec<usize>,
}

impl Frame {
    pub fn new(width: u16, height: u16, bg: Option<crossterm::style::Color>) -> Self {
        let len = width as usize * height as usize;
        Self {
            width,
            height,
            cells: vec![Cell::blank_with_bg(bg); len],
            dirty_all: true,
            dirty_map: vec![false; len],
            dirty: Vec::new(),
        }
    }

    pub fn clear_with_bg(&mut self, bg: Option<crossterm::style::Color>) {
        for cell in &mut self.cells {
            *cell = Cell::blank_with_bg(bg);
        }
        self.dirty_all = true;
        self.dirty.clear();
    }

    pub fn is_dirty_all(&self) -> bool {
        self.dirty_all
    }

    pub fn dirty_indices(&self) -> &[usize] {
        &self.dirty
    }

    pub fn clear_dirty(&mut self) {
        if self.dirty_all {
            self.dirty_all = false;
            self.dirty_map.fill(false);
            self.dirty.clear();
            return;
        }

        for &i in &self.dirty {
            if let Some(v) = self.dirty_map.get_mut(i) {
                *v = false;
            }
        }
        self.dirty.clear();
    }

    pub fn index(&self, x: u16, y: u16) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(y as usize * self.width as usize + x as usize)
    }

    pub fn get(&self, x: u16, y: u16) -> Option<&Cell> {
        self.index(x, y).map(|i| &self.cells[i])
    }

    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        if let Some(i) = self.index(x, y) {
            if self.cells.get(i).copied() == Some(cell) {
                return;
            }

            self.cells[i] = cell;
            if !self.dirty_all && self.dirty_map.get(i).copied() == Some(false) {
                self.dirty_map[i] = true;
                self.dirty.push(i);
            }
        }
    }
}

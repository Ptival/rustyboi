use std::{collections::VecDeque, num::Wrapping};

use crate::{
    ppu::{LCDC_BACKGROUND_TILE_MAP_AREA_BIT, PPU, TILE_MAP_HORIZONTAL_TILE_COUNT},
    utils,
};

use super::{FIFOItem, Fetcher, FetcherState};

#[derive(Clone, Debug)]
pub struct BackgroundOrWindowFetcher {
    state: FetcherState,
    pub fifo: VecDeque<FIFOItem>,
    pub row_of_pixel_within_tile: u8,
    tile_id: u8,
    pub vram_tile_column: u8,
    tile_row_data: [u8; 8],
}

impl BackgroundOrWindowFetcher {
    pub fn new() -> Self {
        BackgroundOrWindowFetcher {
            state: FetcherState::GetTileDelay,
            fifo: VecDeque::new(),
            row_of_pixel_within_tile: 0,
            tile_id: 0,
            vram_tile_column: 0,
            tile_row_data: [0; 8],
        }
    }

    pub fn prepare_for_new_frame(&mut self) {
        self.state = FetcherState::GetTileDelay;
        self.fifo.clear();
        self.row_of_pixel_within_tile = 0;
        self.vram_tile_column = 0;
        self.tile_row_data = [0; 8];
    }

    pub fn prepare_for_new_row(&mut self) {
        self.state = FetcherState::GetTileDelay;
        self.fifo.clear();
        self.row_of_pixel_within_tile = 0;
        self.vram_tile_column = 0;
        self.tile_row_data = [0; 8];
    }

    pub fn tick(&mut self, ppu: &mut PPU) {
        match self.state {
            FetcherState::GetTileDelay => self.state = FetcherState::GetTile,

            FetcherState::GetTile => {
                // NOTE: Because the following operations are done via Wrapping at u8, they
                // automatically perform the necessary "mod 256"
                let vram_pixel_row = (ppu.read_ly() + ppu.scy).0;
                let vram_pixel_col = (Wrapping(self.vram_tile_column) * Wrapping(8) + ppu.scx).0;

                let tile_row = vram_pixel_row / 8;
                let tile_col = vram_pixel_col / 8;

                let tile_index_in_its_tile_map =
                    tile_row as usize * TILE_MAP_HORIZONTAL_TILE_COUNT + tile_col as usize;

                // FIXME: more complex rules for the row base address
                let vram_base_address =
                    if utils::is_bit_set(&ppu.lcd_control, LCDC_BACKGROUND_TILE_MAP_AREA_BIT) {
                        ppu.tile_map0_last_addressing_modes[tile_index_in_its_tile_map] =
                            ppu.get_addressing_mode();
                        0x1C00 // 0x9C00, but VRAM starts at 0x8000
                    } else {
                        ppu.tile_map1_last_addressing_modes[tile_index_in_its_tile_map] =
                            ppu.get_addressing_mode();
                        0x1800 // 0x9800, but VRAM starts at 0x8000
                    };

                let row_address = vram_base_address + ((tile_row as u16) << 5) + (tile_col as u16);

                self.tile_id = ppu.vram[row_address as usize];
                self.state = FetcherState::GetTileDataLowDelay;
            }

            FetcherState::GetTileDataLowDelay => {
                self.state = FetcherState::GetTileDataLow;
            }

            FetcherState::GetTileDataLow => {
                let ly = ppu.read_ly();
                Fetcher::read_tile_row(
                    &ppu.vram,
                    &ppu.get_addressing_mode(),
                    (ly + ppu.scy).0,
                    self.tile_id,
                    false,
                    &mut self.tile_row_data,
                );
                self.state = FetcherState::GetTileDataHighDelay;
            }

            FetcherState::GetTileDataHighDelay => {
                self.state = FetcherState::GetTileDataHigh;
            }

            FetcherState::GetTileDataHigh => {
                let ly = ppu.read_ly();
                Fetcher::read_tile_row(
                    &ppu.vram,
                    &ppu.get_addressing_mode(),
                    (ly + ppu.scy).0,
                    self.tile_id,
                    true,
                    &mut self.tile_row_data,
                );
                self.state = FetcherState::PushRow;
            }

            FetcherState::PushRow => {
                // Background/Window FIFO pixels only get pushed when the FIFO is empty
                if self.fifo.len() == 0 {
                    for i in 0..8 {
                        let color = self.tile_row_data[i];
                        self.fifo.push_back(FIFOItem { color });
                    }
                    self.vram_tile_column += 1;
                    // clean up so that GetTileData can assume 0
                    self.tile_row_data = [0; 8];
                    self.state = FetcherState::GetTileDelay;
                }
            }
        }
    }
}

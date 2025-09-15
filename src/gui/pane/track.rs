use std::error::Error;

use eframe::egui;
use eframe::egui::Pos2;

use lib::score::{ScoreEnvelope, ScoreNote, ScorePart, ScorePartSource};

use crate::pane::Pane;
use crate::pane::sound_source::SoundSourcePane;


const NUM_SCALE: usize = 12;

#[derive(Clone, Copy)]
enum Scale {
    CMajor,
}

impl From<Scale> for [bool; NUM_SCALE] {
    fn from(scale: Scale) -> Self {
        match scale {
            Scale::CMajor => [
                true, false, true, false, true, true,
                false, true, false, true, false, true
            ],
        }
    }
}

#[derive(Clone, Copy)]
pub struct EnvelopePane {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
}

impl EnvelopePane {
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Self {
            attack,
            decay,
            sustain,
            release,
        }
    }
}

impl Default for EnvelopePane {
    fn default() -> Self {
        Self::new(0f32, 0f32, 1f32, 0f32)
    }
}

impl Pane for EnvelopePane {
    fn ui(&mut self, ui: &mut egui::Ui, _: &egui::Context) {
        egui::CollapsingHeader::new("Envelope")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Attack");
                    ui.add(egui::DragValue::new(&mut self.attack)
                        .speed(0.1)
                        .range(0.0..=f32::INFINITY)
                    );

                    ui.label("Decay");
                    ui.add(egui::DragValue::new(&mut self.decay)
                        .speed(0.1)
                        .range(0.0..=f32::INFINITY)
                    );

                    ui.label("Sustain");
                    ui.add(egui::DragValue::new(&mut self.sustain)
                        .speed(0.1)
                        .range(0.0..=1.0f32)
                    );

                    ui.label("Release");
                    ui.add(egui::DragValue::new(&mut self.release)
                        .speed(0.1)
                        .range(0.0..=1.0f32)
                    );
                });
            });
    }
}

impl From<EnvelopePane> for ScoreEnvelope {
    fn from(envelope_pane: EnvelopePane) -> Self {
        Self::new(
            envelope_pane.attack,
            envelope_pane.decay,
            envelope_pane.sustain,
            envelope_pane.release,
        )
    }
}

impl From<&ScoreEnvelope> for EnvelopePane {
    fn from(score_envelope: &ScoreEnvelope) -> EnvelopePane {
        Self::new(
            score_envelope.attack(),
            score_envelope.decay(),
            score_envelope.sustain(),
            score_envelope.release(),
        )
    }
}

#[derive(Clone, Copy)]
struct NoteTile {
    semitone: f32,
    start: f32,
    length: f32,
    envelope: Option<EnvelopePane>,
    is_property_displayed: bool
}

impl NoteTile {
    fn new(semitone: f32, start: f32, length: f32, envelope: Option<EnvelopePane>) -> Self {
        Self {
            semitone,
            start,
            length,
            envelope,
            is_property_displayed: false,
        }
    }
}

impl Pane for NoteTile {
    fn ui(&mut self, _: &mut egui::Ui, ctx: &egui::Context) {
        if self.is_property_displayed {
            egui::Window::new("Property")
                .open(&mut self.is_property_displayed)
                .show(ctx, |ui| {
                    ui.label("Property");
                    ui.horizontal(|ui| {
                        ui.label("Semitone");
                        ui.add(egui::DragValue::new(&mut self.semitone)
                            .speed(0.1)
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("start");
                        ui.add(egui::DragValue::new(&mut self.start)
                            .speed(0.1)
                            .range(0.0..=f32::INFINITY)
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("length");
                        ui.add(egui::DragValue::new(&mut self.length)
                            .speed(0.1)
                            .range(0.0..=f32::INFINITY)
                        );
                    });

                    match self.envelope.as_mut() {
                        Some(envelope) => {
                            envelope.ui(ui, ctx);
                            if ui.button("Remove Envelope").clicked() {
                                self.envelope = None;
                            }
                        },
                        None => {
                            if ui.button("Add Envelope").clicked() {
                                self.envelope = Some(EnvelopePane::default());
                            }
                        },
                    }
                });
        }
    }
}

impl From<NoteTile> for ScoreNote {
    fn from(note_tile: NoteTile) -> Self {
        Self::new(
            Some(note_tile.semitone),
            Some(note_tile.start),
            note_tile.length,
            match note_tile.envelope {
                Some(envelope) => Some(envelope.into()),
                None => None,
            },
        )
    }
}

pub struct GridRegion {
    height_num: f32,

    width_unit: f32,
    height_unit: f32,

    offset: Pos2,

    width_move_unit: Option<f32>,
    height_move_unit: Option<f32>,

    painter: Option<egui::Painter>,
    rect: Option<egui::Rect>,
    response: Option<egui::Response>,
}

impl GridRegion {
    fn new(height_num: f32, width_unit: f32, height_unit: f32,
        width_offset: f32, height_offset: f32,
        width_move_unit: Option<f32>, height_move_unit: Option<f32>) -> Self {
        Self {
            height_num,
            width_unit,
            height_unit,
            offset: Pos2::new(width_offset, height_offset),
            width_move_unit,
            height_move_unit,
            painter: None,
            rect: None,
            response: None,
        }
    }

    fn top(&self) -> f32 {
        match self.rect.as_ref() {
            Some(rect) => rect.top(),
            None => 0f32,
        }
    }

    fn left(&self) -> f32 {
        match self.rect.as_ref() {
            Some(rect) => rect.left(),
            None => 0f32,
        }
    }

    fn display_to_true_position(&self, pos: Pos2) -> Pos2 {
        Pos2::new(pos.x - self.left() + self.offset.x, -pos.y + self.top() - self.offset.y)
    }

    fn true_to_display_position(&self, pos: Pos2) -> Pos2 {
        Pos2::new(pos.x + self.left() - self.offset.x, -pos.y + self.top() - self.offset.y)
    }

    fn get_snapped_grid_pos(&self, pos: Pos2) -> Pos2 {
        let gx = match self.width_move_unit {
            Some(wo) => (pos.x / self.width_unit / wo).floor() * wo,
            None => (pos.x / self.width_unit).floor(),
        };
        let gy = match self.height_move_unit {
            Some(ho) => (pos.y / self.height_unit / ho).floor() * ho,
            None => (pos.y / self.height_unit).floor(),
        };
        Pos2::new(gx, gy)
    }

    fn get_rect(&self, pos: Pos2, width: f32, height: f32) -> Option<egui::Rect> {
        if self.rect.is_none() {
            return None;
        }
        let rect = self.rect.unwrap();

        let left_bottom = self.true_to_display_position(pos);
        let left = left_bottom.x;
        let right = left + width;
        let bottom = left_bottom.y;
        let top = bottom - height;

        if right < rect.left() || rect.right() < left || bottom < rect.top() || rect.bottom() < top {
            return None;
        }

        let left_top = Pos2::new(left.max(rect.left()), top.max(rect.top()));
        let width_height = egui::Vec2::new(
            right.min(rect.right()) - left.max(rect.left()),
            bottom.min(rect.bottom()) - top.max(rect.top())
        );

        Some(egui::Rect::from_min_size(
            left_top,
            width_height
        ))
    }

    fn rect_filled(&self, rect: egui::Rect,
        corner_radius: impl Into<egui::CornerRadius>, fill_color: impl Into<egui::Color32>) {
        if self.painter.is_none() {
            return;
        }
        let painter = self.painter.as_ref().unwrap();
        painter.rect_filled(rect, corner_radius, fill_color);
    }
}

impl Pane for GridRegion {
    fn ui(&mut self, ui: &mut egui::Ui, _: &egui::Context) {
        let (rect, response) = ui.allocate_exact_size(
            egui::vec2(ui.available_width(), self.height_num * self.height_unit),
            // egui::vec2(self.width_num * self.width_unit, self.height_num * self.height_unit),
            egui::Sense::click_and_drag()
        );

        self.rect = Some(rect);

        let painter = ui.painter_at(rect);

        let true_left_bottom = self.display_to_true_position(Pos2::new(rect.left(), rect.bottom()));
        if let Some(height_move_unit) = self.height_move_unit {
            let pos = self.true_to_display_position(Pos2::new(
                    0f32,
                    (true_left_bottom.y / self.height_unit / height_move_unit).floor()
                        * self.height_unit * height_move_unit));
            let mut y = pos.y;
            while rect.top() <= y {
                painter.line_segment(
                    [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                    egui::Stroke::new(0.5f32, egui::Color32::LIGHT_GRAY),
                );
                y -= height_move_unit * self.height_unit;
            }
        }

        if let Some(width_move_unit) = self.width_move_unit {
            let pos = self.true_to_display_position(Pos2::new(
                    (self.offset.x / self.width_unit / width_move_unit).floor() * self.width_unit * width_move_unit,
                    0f32));
            let mut x = pos.x;
            while x <= rect.right() {
                painter.line_segment(
                    [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
                    egui::Stroke::new(0.5f32, egui::Color32::LIGHT_GRAY),
                );
                x += width_move_unit * self.width_unit;
            }
        }

        let pos = self.true_to_display_position(Pos2::new(
                0f32,
                (true_left_bottom.y / self.height_unit).floor() * self.height_unit));
        let mut y = pos.y;
        while rect.top() <= y {
            painter.line_segment(
                [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                egui::Stroke::new(1f32, egui::Color32::GRAY),
            );
            y -= self.height_unit;
        }

        self.painter = Some(painter);
        self.response = Some(response);
    }
}

pub struct TrackPane {
    source: SoundSourcePane,
    bpm: f32,
    source_notes: Vec<NoteTile>,
    volume: f32,
    channel: u16,
    max_channel: u16,
    envelope: Option<EnvelopePane>,

    scale: Scale,

    semitone_height: f32,
    num_display_semitone: f32,
    display_semitone_offset: f32,
    semitone_move_unit: f32,

    beat_width: f32,
    display_beat_offset: f32,
    beat_move_unit: f32,
}

impl Default for TrackPane {
    fn default() -> Self {
        Self {
            source: SoundSourcePane::new(ScorePartSource::Sin),
            bpm: Self::DEFAULT_BPM,
            source_notes: Vec::new(),
            volume: Self::DEFAULT_VOLUME,
            channel: Self::DEFAULT_CHANNEL,
            max_channel: Self::DEFAULT_CHANNEL+1,
            envelope: None,
            scale: Self::DEFAULT_SCALE,

            semitone_height: Self::DEFAULT_SEMITONE_HEIGHT,
            num_display_semitone: Self::DEFAULT_NUM_SEMITONE,
            display_semitone_offset: Self::DEFAULT_SEMITONE_OFFSET,
            semitone_move_unit: Self::DEFAULT_SEMITONE_MOVE_UNIT,

            beat_width: Self::DEFAULT_BEAT_WIDTH,
            display_beat_offset: Self::DEFAULT_BEAT_OFFSET,
            beat_move_unit: Self::DEFAULT_BEAT_MOVE_UNIT,
        }
    }
}

impl TrackPane {
    const DEFAULT_BPM: f32 = 60f32;
    const DEFAULT_VOLUME: f32 = 1f32;
    const DEFAULT_CHANNEL: u16 = 0u16;
    const DEFAULT_SCALE: Scale = Scale::CMajor;

    const DEFAULT_SEMITONE_HEIGHT: f32 = 20f32;
    const DEFAULT_NUM_SEMITONE: f32 = 13f32;
    const DEFAULT_SEMITONE_OFFSET: f32 = - Self::DEFAULT_NUM_SEMITONE * Self::DEFAULT_SEMITONE_HEIGHT;
    const DEFAULT_SEMITONE_MOVE_UNIT: f32 = 1.0f32;

    const DEFAULT_BEAT_WIDTH: f32 = 40f32;
    const DEFAULT_BEAT_OFFSET: f32 = 0f32;
    const DEFAULT_BEAT_MOVE_UNIT: f32 = 1.0f32;

    pub fn new(max_channel: u16) -> Self {
        Self {
            max_channel,
            ..Default::default()
        }
    }

    pub fn set_max_channel(&mut self, max_channel: u16) {
        self.max_channel = max_channel;
    }

    pub fn from_score_part(score_part: &ScorePart, max_channel: u16) -> Result<Self, Box<dyn Error>> {
        let volume = match score_part.volume() {
            Some(volume) => volume,
            None => TrackPane::DEFAULT_VOLUME,
        };
        let mut start = 0f32;
        let mut source_notes = vec![];
        for score_note in score_part.score_notes().iter() {
            let current_start = match score_note.start() {
                Some(current_start) => current_start,
                None => start,
            };
            if let Some(semitone) = score_note.semitone() {
                source_notes.push(NoteTile::new(
                    semitone,
                    current_start,
                    score_note.length(),
                    match score_note.envelope() {
                        Some(envelope) => Some(envelope.into()),
                        None => None,
                    },
                ));
            }
            start = start.max(current_start + score_note.length());
        }
        Ok(Self {
            source: SoundSourcePane::new(score_part.source()),
            bpm: score_part.bpm(),
            source_notes,
            volume,
            channel: score_part.channel(),
            max_channel,
            ..Default::default()
        })
    }
}

impl Pane for TrackPane {
    fn ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.horizontal(|ui| {
            ui.label("BPM");
            ui.add(egui::DragValue::new(&mut self.bpm)
                .range(0f32..=f32::INFINITY));
            ui.label("Volume");
            ui.add(egui::DragValue::new(&mut self.volume)
                .range(0f32..=f32::INFINITY));
            ui.label("Channel");
            ui.add(egui::DragValue::new(&mut self.channel)
                .range(0u16..=self.max_channel-1));
            ui.label("Beat Length");
            ui.add(egui::DragValue::new(&mut self.beat_move_unit)
                .range(0f32..=f32::INFINITY));
        });

        match self.envelope.as_mut() {
            Some(envelope) => {
                envelope.ui(ui, ctx);
                if ui.button("Remove Envelope").clicked() {
                    self.envelope = None;
                }
            },
            None => {
                if ui.button("Add Envelope").clicked() {
                    self.envelope = Some(EnvelopePane::default());
                }
            },
        }

        ui.horizontal(|ui| {
            ui.label("Source");
            self.source.ui(ui, ctx);
        });

        ui.horizontal(|ui| {
            ui.label("Start");
            ui.add(egui::DragValue::new(&mut self.display_beat_offset)
                .range(0f32..=f32::INFINITY)
                .custom_formatter(|n, _| format!("{}",
                        (n as f32 / self.beat_width / self.beat_move_unit).floor() * self.beat_move_unit)));
        });

        let mut grid_region = GridRegion::new(
            self.num_display_semitone,
            self.beat_width, self.semitone_height,
            self.display_beat_offset, self.display_semitone_offset,
            Some(self.beat_move_unit), Some(self.semitone_move_unit)
        );
        grid_region.ui(ui, ctx);

        if let Some(grid_region_rect) = grid_region.rect {
            let true_left_bottom = grid_region.display_to_true_position(Pos2::new(
                    grid_region_rect.left(), grid_region_rect.bottom()
            ));
            let true_right_top = grid_region.display_to_true_position(Pos2::new(
                    grid_region_rect.right(), grid_region_rect.top()
            ));
            let mut semitone_y_index = (true_left_bottom.y / self.semitone_height).floor() as i64;
            let scale_tones: [bool; 12] = self.scale.into();

            let left = true_left_bottom.x;
            let top = true_right_top.y;
            while semitone_y_index as f32 * self.semitone_height <= top {
                let index = (NUM_SCALE as i64 + (semitone_y_index % NUM_SCALE as i64)) as usize % NUM_SCALE;
                let semitone_y = semitone_y_index as f32 * self.semitone_height;
                if !scale_tones[index] {
                    if let Some(rect) = grid_region.get_rect(
                        Pos2::new(left, semitone_y), grid_region_rect.width(), self.semitone_height
                    ) {
                        grid_region.rect_filled(rect, 2f32, egui::Color32::LIGHT_GRAY);
                    }
                }

                if index == 0 {
                    if let Some(painter) = grid_region.painter.as_ref() {
                        let zero_semitone = grid_region.true_to_display_position(Pos2::new(0f32, semitone_y)).y;
                        let (bold, color) = if semitone_y_index == 0 {
                            (2f32, egui::Color32::RED)
                        } else {
                            (1f32, egui::Color32::BLACK)
                        };
                        painter.line_segment([
                            Pos2::new(grid_region_rect.left(), zero_semitone),
                            Pos2::new(grid_region_rect.right(), zero_semitone)
                        ], egui::Stroke::new(bold, color));
                    }
                }

                semitone_y_index += 1;
            }
        }

        let mut delete_note_ids = vec![];
        for (i, note) in self.source_notes.iter_mut().enumerate() {
            if let Some(note_rect) = grid_region.get_rect(
                Pos2::new(note.start * self.beat_width, note.semitone * self.semitone_height),
                self.beat_width * note.length,
                self.semitone_height,
            ) {
                let note_id = ui.make_persistent_id(i);
                let resp = ui.interact(note_rect, note_id, egui::Sense::click_and_drag());
                grid_region.rect_filled(note_rect, 2f32, egui::Color32::LIGHT_BLUE);
                if resp.dragged() {
                    let delta = resp.drag_delta();
                    note.start += delta.x / self.beat_width;
                    note.semitone -= delta.y / self.semitone_height;
                }
                if resp.drag_stopped() {
                    let note_pos = grid_region.get_snapped_grid_pos(Pos2::new(
                            note.start * self.beat_width,
                            note.semitone * self.semitone_height
                    ));
                    note.start = note_pos.x;
                    note.semitone = note_pos.y;
                }
                resp.context_menu(|ui| {
                    if ui.button("Delete").clicked() {
                        delete_note_ids.push(i);
                    }
                    if ui.button("Property").clicked() {
                        note.is_property_displayed = true;
                    }
                });
                note.ui(ui, ctx);
            }
        }
        for id in delete_note_ids {
            self.source_notes.remove(id);
        }

        if let Some(grid_region_response) = grid_region.response.as_ref() {
            if grid_region_response.clicked() {
                if let Some(pos) = grid_region_response.interact_pointer_pos() {
                    let note_pos = grid_region.get_snapped_grid_pos(
                        grid_region.display_to_true_position(pos)
                    );
                    self.source_notes.push(NoteTile::new(note_pos.y, note_pos.x, self.beat_move_unit, None));
                }
            }

            if grid_region_response.dragged_by(egui::PointerButton::Middle) {
                let delta = grid_region_response.drag_delta();
                self.display_beat_offset -= delta.x;
                self.display_beat_offset = self.display_beat_offset.max(0f32);
                self.display_semitone_offset -= delta.y;
            }
        }
    }
}

impl From<&TrackPane> for ScorePart {
    fn from(track_pane: &TrackPane) -> Self {
        let source = track_pane.source.source().to_owned();
        Self::new(
            source,
            track_pane.bpm,
            track_pane.source_notes.iter().map(|&source_note| source_note.into()).collect::<Vec<ScoreNote>>(),
            Some(track_pane.volume),
            Some(track_pane.channel),
            match track_pane.envelope {
                Some(envelope) => Some(envelope.into()),
                None => None,
            },
        )
    }
}

use ratatui::{
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    text::Span,
    widgets::{Block, Borders, HighlightSpacing, Row, Table, TableState},
    Frame,
};

use crate::{exercise::Exercise, state_file::StateFile};

pub struct UiState<'a> {
    pub table: Table<'a>,
    selected: usize,
    table_state: TableState,
    last_ind: usize,
}

impl<'a> UiState<'a> {
    pub fn rows<'s, 'i>(
        state_file: &'s StateFile,
        exercises: &'a [Exercise],
    ) -> impl Iterator<Item = Row<'a>> + 'i
    where
        's: 'i,
        'a: 'i,
    {
        exercises
            .iter()
            .zip(state_file.progress())
            .enumerate()
            .map(|(ind, (exercise, done))| {
                let next = if ind == state_file.next_exercise_ind() {
                    ">>>>".bold().red()
                } else {
                    Span::default()
                };

                let exercise_state = if *done {
                    "DONE".green()
                } else {
                    "PENDING".yellow()
                };

                Row::new([
                    next,
                    exercise_state,
                    Span::raw(&exercise.name),
                    Span::raw(exercise.path.to_string_lossy()),
                ])
            })
    }

    pub fn new(state_file: &StateFile, exercises: &'a [Exercise]) -> Self {
        let header = Row::new(["Next", "State", "Name", "Path"]);

        let max_name_len = exercises
            .iter()
            .map(|exercise| exercise.name.len())
            .max()
            .unwrap_or(4) as u16;

        let widths = [
            Constraint::Length(4),
            Constraint::Length(7),
            Constraint::Length(max_name_len),
            Constraint::Fill(1),
        ];

        let rows = Self::rows(state_file, exercises);

        let table = Table::new(rows, widths)
            .header(header)
            .column_spacing(2)
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_style(Style::new().bg(ratatui::style::Color::Rgb(50, 50, 50)))
            .highlight_symbol("🦀")
            .block(Block::default().borders(Borders::BOTTOM));

        let selected = 0;
        let table_state = TableState::default().with_selected(Some(selected));
        let last_ind = exercises.len() - 1;

        Self {
            table,
            selected,
            table_state,
            last_ind,
        }
    }

    #[inline]
    pub fn selected(&self) -> usize {
        self.selected
    }

    fn select(&mut self, ind: usize) {
        self.selected = ind;
        self.table_state.select(Some(ind));
    }

    pub fn select_next(&mut self) {
        self.select(self.selected.saturating_add(1).min(self.last_ind));
    }

    pub fn select_previous(&mut self) {
        self.select(self.selected.saturating_sub(1));
    }

    #[inline]
    pub fn select_first(&mut self) {
        self.select(0);
    }

    #[inline]
    pub fn select_last(&mut self) {
        self.select(self.last_ind);
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let area = frame.size();

        frame.render_stateful_widget(
            &self.table,
            Rect {
                x: 0,
                y: 0,
                width: area.width,
                height: area.height - 1,
            },
            &mut self.table_state,
        );

        let help_footer =
            "↓/j ↑/k home/g end/G │ Filter <d>one/<p>ending │ <r>eset │ <c>ontinue at │ <q>uit";
        frame.render_widget(
            Span::raw(help_footer),
            Rect {
                x: 0,
                y: area.height - 1,
                width: area.width,
                height: 1,
            },
        );
    }
}
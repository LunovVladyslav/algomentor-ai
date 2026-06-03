use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use ratatui::widgets::*;
use std::io;

use crate::memory::database::Database;
use crate::memory::profile::{UserProfile, UserStats};
use crate::task::discovery;
use crate::task::models::TaskInfo;

use super::charts;

/// Active tab in the dashboard
#[derive(Debug, Clone, Copy, PartialEq)]
enum Tab {
    Overview,
    Tasks,
    Analytics,
}

/// Dashboard application state
pub struct DashboardApp {
    stats: UserStats,
    tasks: Vec<TaskInfo>,
    db_tasks: Vec<crate::task::models::Task>,
    active_tab: Tab,
    task_scroll: usize,
    should_quit: bool,
}

impl DashboardApp {
    pub fn new(db: &Database, project_dir: &std::path::Path) -> Result<Self> {
        let profile = UserProfile::new(db);
        let stats = profile.get_stats()?;
        let tasks = discovery::discover_tasks(project_dir, 3)?;

        let db_tasks = db.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, name, directory, difficulty, category, status, started_at, completed_at, time_complexity, space_complexity, attempts, language FROM tasks ORDER BY started_at DESC"
            )?;
            let tasks = stmt.query_map([], |row| {
                Ok(crate::task::models::Task {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    directory: row.get(2)?,
                    difficulty: row.get(3)?,
                    category: row.get(4)?,
                    status: row.get(5)?,
                    started_at: row.get(6)?,
                    completed_at: row.get(7)?,
                    time_complexity: row.get(8)?,
                    space_complexity: row.get(9)?,
                    attempts: row.get(10)?,
                    language: row.get(11)?,
                })
            })?.collect::<std::result::Result<Vec<_>, _>>()?;
            Ok(tasks)
        }).unwrap_or_default();

        Ok(Self {
            stats,
            tasks,
            db_tasks,
            active_tab: Tab::Overview,
            task_scroll: 0,
            should_quit: false,
        })
    }

    /// Run the TUI dashboard
    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        while !self.should_quit {
            terminal.draw(|f| self.render(f))?;
            self.handle_events()?;
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                        KeyCode::Tab => {
                            self.active_tab = match self.active_tab {
                                Tab::Overview => Tab::Tasks,
                                Tab::Tasks => Tab::Analytics,
                                Tab::Analytics => Tab::Overview,
                            };
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if self.task_scroll < self.tasks.len().saturating_sub(1) {
                                self.task_scroll += 1;
                            }
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            self.task_scroll = self.task_scroll.saturating_sub(1);
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(1),
            ])
            .split(area);

        self.render_header(frame, chunks[0]);
        self.render_tabs(frame, chunks[1]);

        match self.active_tab {
            Tab::Overview => self.render_overview(frame, chunks[2]),
            Tab::Tasks => self.render_tasks(frame, chunks[2]),
            Tab::Analytics => self.render_analytics(frame, chunks[2]),
        }

        self.render_footer(frame, chunks[3]);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let header = Paragraph::new("🧠 AlgoMentor Dashboard")
            .style(Style::default().fg(Color::Cyan).bold())
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .border_style(Style::default().fg(Color::DarkGray)),
            );
        frame.render_widget(header, area);
    }

    fn render_tabs(&self, frame: &mut Frame, area: Rect) {
        let tabs = Tabs::new(vec!["Overview", "Tasks", "Analytics"])
            .select(match self.active_tab {
                Tab::Overview => 0,
                Tab::Tasks => 1,
                Tab::Analytics => 2,
            })
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Cyan).bold().underlined())
            .divider(" │ ")
            .block(Block::default().borders(Borders::BOTTOM).border_style(Style::default().fg(Color::DarkGray)));
        frame.render_widget(tabs, area);
    }

    fn render_overview(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(8), Constraint::Min(5)])
            .split(chunks[0]);

        let total = self.stats.total_tasks.max(1) as f64;
        let ratio = self.stats.completed_tasks as f64 / total;

        let progress_text = vec![
            Line::from(vec![
                Span::styled("Solved: ", Style::default().fg(Color::Green)),
                Span::styled(
                    format!("{}", self.stats.completed_tasks),
                    Style::default().fg(Color::Green).bold(),
                ),
                Span::raw("  "),
                Span::styled("In Progress: ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    format!("{}", self.stats.in_progress_tasks),
                    Style::default().fg(Color::Yellow).bold(),
                ),
                Span::raw("  "),
                Span::styled("Abandoned: ", Style::default().fg(Color::Red)),
                Span::styled(
                    format!("{}", self.stats.abandoned_tasks),
                    Style::default().fg(Color::Red).bold(),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Total Sessions: ", Style::default().fg(Color::White)),
                Span::styled(
                    format!("{}", self.stats.total_sessions),
                    Style::default().fg(Color::Cyan).bold(),
                ),
                Span::raw("  "),
                Span::styled("Messages: ", Style::default().fg(Color::White)),
                Span::styled(
                    format!("{}", self.stats.total_messages),
                    Style::default().fg(Color::Cyan).bold(),
                ),
            ]),
        ];

        let progress = Paragraph::new(progress_text).block(
            Block::default()
                .title(" 📊 Progress Overview ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        );
        frame.render_widget(progress, left_chunks[0]);

        let gauge = Gauge::default()
            .block(
                Block::default()
                    .title(format!(" Completion: {:.0}% ", ratio * 100.0))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .gauge_style(Style::default().fg(Color::Green).bg(Color::DarkGray))
            .ratio(ratio.min(1.0));
        frame.render_widget(gauge, left_chunks[1]);

        // Right: Strengths & Weaknesses
        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        let strengths: Vec<Line> = if self.stats.strengths.is_empty() {
            vec![Line::from(Span::styled("No data yet", Style::default().fg(Color::DarkGray)))]
        } else {
            self.stats.strengths.iter().map(|s| {
                Line::from(format!("  💪 {}", s))
            }).collect()
        };

        let strengths_widget = Paragraph::new(strengths).block(
            Block::default()
                .title(" 💪 Strong Areas ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green)),
        );
        frame.render_widget(strengths_widget, right_chunks[0]);

        let weaknesses: Vec<Line> = if self.stats.weaknesses.is_empty() {
            vec![Line::from(Span::styled("No data yet", Style::default().fg(Color::DarkGray)))]
        } else {
            self.stats.weaknesses.iter().map(|s| {
                Line::from(format!("  🎯 {}", s))
            }).collect()
        };

        let weaknesses_widget = Paragraph::new(weaknesses).block(
            Block::default()
                .title(" 🎯 Areas to Improve ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        );
        frame.render_widget(weaknesses_widget, right_chunks[1]);
    }

    fn render_tasks(&self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .tasks
            .iter()
            .enumerate()
            .map(|(i, task)| {
                let status_icon = if task.solution_files.is_empty() { "📝" } else { "🔄" };

                let title = task
                    .description
                    .as_ref()
                    .map(|d| d.title.as_str())
                    .unwrap_or(&task.name);

                let difficulty = task
                    .description
                    .as_ref()
                    .and_then(|d| d.difficulty.as_ref())
                    .map(|d| format!(" [{}]", d))
                    .unwrap_or_default();

                let category = task
                    .description
                    .as_ref()
                    .and_then(|d| d.category.as_ref())
                    .map(|c| format!(" ({})", c))
                    .unwrap_or_default();

                let text = format!(
                    " {} {}{}{}  ({} files)",
                    status_icon, title, difficulty, category,
                    task.solution_files.len()
                );

                let style = if i == self.task_scroll {
                    Style::default().bg(Color::DarkGray)
                } else {
                    Style::default()
                };

                ListItem::new(text).style(style)
            })
            .collect();

        let tasks_list = List::new(items)
            .block(
                Block::default()
                    .title(format!(" 📋 Tasks ({}) ", self.tasks.len()))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .highlight_style(Style::default().bg(Color::DarkGray));

        frame.render_widget(tasks_list, area);
    }

    fn render_analytics(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Complexity distribution
        let complexity_data = charts::get_complexity_distribution(&self.db_tasks);
        let bars: Vec<Bar> = complexity_data
            .iter()
            .map(|(label, count)| {
                Bar::default()
                    .label(Line::from(label.as_str()))
                    .value(*count as u64)
                    .style(Style::default().fg(Color::Cyan))
            })
            .collect();

        let chart = BarChart::default()
            .block(
                Block::default()
                    .title(" 📈 Complexity Distribution ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .data(BarGroup::default().bars(&bars))
            .bar_width(8)
            .bar_gap(2)
            .value_style(Style::default().fg(Color::White).bold());

        frame.render_widget(chart, chunks[0]);

        // Category progress
        let category_data = charts::get_category_progress(&self.db_tasks);
        let cat_items: Vec<ListItem> = category_data
            .iter()
            .map(|(cat, solved, total)| {
                let bar_len = 20;
                let filled = if *total > 0 {
                    ((*solved as f64 / *total as f64) * bar_len as f64) as usize
                } else {
                    0
                };

                let text = format!(
                    " {:<20} {}{} {}/{}",
                    cat,
                    "█".repeat(filled),
                    "░".repeat(bar_len - filled),
                    solved,
                    total
                );

                ListItem::new(text)
            })
            .collect();

        let cat_list = List::new(cat_items).block(
            Block::default()
                .title(" 📊 Category Progress ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green)),
        );

        frame.render_widget(cat_list, chunks[1]);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let footer = Paragraph::new(
            " Tab: Switch view │ ↑↓: Navigate │ q: Quit "
        )
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
        frame.render_widget(footer, area);
    }
}

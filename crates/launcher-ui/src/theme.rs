//! Extensible theming system.
//!
//! Themes are CSS-variable-based so consumers can override colors, spacing,
//! fonts, and radii without touching component code. The `Theme` struct
//! generates a `:root { ... }` CSS block that components reference via `var(--x)`.
//!
//! Library consumers provide a custom `Theme` to restyle the entire launcher.

/// Complete theme definition. All visual properties of the launcher.
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub name: String,

    // Colors
    pub bg_primary: String,
    pub bg_secondary: String,
    pub bg_tertiary: String,
    pub bg_hover: String,
    pub bg_selected: String,
    pub bg_input: String,

    pub text_primary: String,
    pub text_secondary: String,
    pub text_muted: String,
    pub text_inverse: String,

    pub accent: String,
    pub accent_hover: String,
    pub accent_dim: String,

    pub border: String,
    pub border_focus: String,

    pub match_highlight: String,
    pub match_highlight_bg: String,

    pub tag_bg: String,
    pub tag_text: String,
    pub tag_border: String,

    pub success: String,
    pub warning: String,
    pub error: String,

    // Spacing
    pub spacing_xs: String,
    pub spacing_sm: String,
    pub spacing_md: String,
    pub spacing_lg: String,
    pub spacing_xl: String,

    // Typography
    pub font_family: String,
    pub font_mono: String,
    pub font_size_xs: String,
    pub font_size_sm: String,
    pub font_size_md: String,
    pub font_size_lg: String,
    pub font_size_xl: String,
    pub font_weight_normal: String,
    pub font_weight_medium: String,
    pub font_weight_bold: String,
    pub line_height: String,

    // Borders & Radii
    pub radius_sm: String,
    pub radius_md: String,
    pub radius_lg: String,
    pub radius_full: String,

    // Shadows
    pub shadow_sm: String,
    pub shadow_md: String,
    pub shadow_lg: String,

    // Transitions
    pub transition_fast: String,
    pub transition_normal: String,

    // Layout
    pub sidebar_width: String,
    pub search_height: String,
    pub result_item_height: String,
    pub action_bar_height: String,

    /// Additional CSS appended after the theme variables.
    /// Use this to inject component overrides or custom styles.
    pub custom_css: String,
}

impl Theme {
    /// Catppuccin Mocha dark theme (default).
    pub fn dark() -> Self {
        Self {
            name: "dark".into(),

            bg_primary: "#1e1e2e".into(),
            bg_secondary: "#313244".into(),
            bg_tertiary: "#181825".into(),
            bg_hover: "#45475a".into(),
            bg_selected: "#585b70".into(),
            bg_input: "#242438".into(),

            text_primary: "#cdd6f4".into(),
            text_secondary: "#a6adc8".into(),
            text_muted: "#6c7086".into(),
            text_inverse: "#1e1e2e".into(),

            accent: "#89b4fa".into(),
            accent_hover: "#74c7ec".into(),
            accent_dim: "#45475a".into(),

            border: "#45475a".into(),
            border_focus: "#89b4fa".into(),

            match_highlight: "#f9e2af".into(),
            match_highlight_bg: "rgba(249, 226, 175, 0.1)".into(),

            tag_bg: "#313244".into(),
            tag_text: "#a6adc8".into(),
            tag_border: "#45475a".into(),

            success: "#a6e3a1".into(),
            warning: "#f9e2af".into(),
            error: "#f38ba8".into(),

            spacing_xs: "2px".into(),
            spacing_sm: "4px".into(),
            spacing_md: "8px".into(),
            spacing_lg: "12px".into(),
            spacing_xl: "16px".into(),

            font_family: r#""Inter", "SF Pro Display", system-ui, -apple-system, sans-serif"#
                .into(),
            font_mono: r#""JetBrains Mono", "Fira Code", "SF Mono", monospace"#.into(),
            font_size_xs: "10px".into(),
            font_size_sm: "12px".into(),
            font_size_md: "14px".into(),
            font_size_lg: "16px".into(),
            font_size_xl: "20px".into(),
            font_weight_normal: "400".into(),
            font_weight_medium: "500".into(),
            font_weight_bold: "600".into(),
            line_height: "1.5".into(),

            radius_sm: "4px".into(),
            radius_md: "6px".into(),
            radius_lg: "10px".into(),
            radius_full: "9999px".into(),

            shadow_sm: "0 1px 2px rgba(0,0,0,0.3)".into(),
            shadow_md: "0 4px 12px rgba(0,0,0,0.4)".into(),
            shadow_lg: "0 8px 24px rgba(0,0,0,0.5)".into(),

            transition_fast: "0.1s ease".into(),
            transition_normal: "0.2s ease".into(),

            sidebar_width: "220px".into(),
            search_height: "52px".into(),
            result_item_height: "48px".into(),
            action_bar_height: "36px".into(),

            custom_css: String::new(),
        }
    }

    /// Light theme variant.
    pub fn light() -> Self {
        Self {
            name: "light".into(),

            bg_primary: "#eff1f5".into(),
            bg_secondary: "#e6e9ef".into(),
            bg_tertiary: "#dce0e8".into(),
            bg_hover: "#ccd0da".into(),
            bg_selected: "#bcc0cc".into(),
            bg_input: "#e6e9ef".into(),

            text_primary: "#4c4f69".into(),
            text_secondary: "#6c6f85".into(),
            text_muted: "#9ca0b0".into(),
            text_inverse: "#eff1f5".into(),

            accent: "#1e66f5".into(),
            accent_hover: "#2a7ae4".into(),
            accent_dim: "#ccd0da".into(),

            border: "#ccd0da".into(),
            border_focus: "#1e66f5".into(),

            match_highlight: "#df8e1d".into(),
            match_highlight_bg: "rgba(223, 142, 29, 0.1)".into(),

            tag_bg: "#e6e9ef".into(),
            tag_text: "#6c6f85".into(),
            tag_border: "#ccd0da".into(),

            success: "#40a02b".into(),
            warning: "#df8e1d".into(),
            error: "#d20f39".into(),

            shadow_sm: "0 1px 2px rgba(0,0,0,0.08)".into(),
            shadow_md: "0 4px 12px rgba(0,0,0,0.1)".into(),
            shadow_lg: "0 8px 24px rgba(0,0,0,0.15)".into(),

            ..Self::dark()
        }
    }

    /// Generate the CSS `:root` block with all variables.
    pub fn to_css_variables(&self) -> String {
        format!(
            r#":root {{
  --bg-primary: {bg_primary};
  --bg-secondary: {bg_secondary};
  --bg-tertiary: {bg_tertiary};
  --bg-hover: {bg_hover};
  --bg-selected: {bg_selected};
  --bg-input: {bg_input};
  --text-primary: {text_primary};
  --text-secondary: {text_secondary};
  --text-muted: {text_muted};
  --text-inverse: {text_inverse};
  --accent: {accent};
  --accent-hover: {accent_hover};
  --accent-dim: {accent_dim};
  --border: {border};
  --border-focus: {border_focus};
  --match-highlight: {match_highlight};
  --match-highlight-bg: {match_highlight_bg};
  --tag-bg: {tag_bg};
  --tag-text: {tag_text};
  --tag-border: {tag_border};
  --success: {success};
  --warning: {warning};
  --error: {error};
  --spacing-xs: {spacing_xs};
  --spacing-sm: {spacing_sm};
  --spacing-md: {spacing_md};
  --spacing-lg: {spacing_lg};
  --spacing-xl: {spacing_xl};
  --font-family: {font_family};
  --font-mono: {font_mono};
  --font-size-xs: {font_size_xs};
  --font-size-sm: {font_size_sm};
  --font-size-md: {font_size_md};
  --font-size-lg: {font_size_lg};
  --font-size-xl: {font_size_xl};
  --font-weight-normal: {font_weight_normal};
  --font-weight-medium: {font_weight_medium};
  --font-weight-bold: {font_weight_bold};
  --line-height: {line_height};
  --radius-sm: {radius_sm};
  --radius-md: {radius_md};
  --radius-lg: {radius_lg};
  --radius-full: {radius_full};
  --shadow-sm: {shadow_sm};
  --shadow-md: {shadow_md};
  --shadow-lg: {shadow_lg};
  --transition-fast: {transition_fast};
  --transition-normal: {transition_normal};
  --sidebar-width: {sidebar_width};
  --search-height: {search_height};
  --result-item-height: {result_item_height};
  --action-bar-height: {action_bar_height};
}}"#,
            bg_primary = self.bg_primary,
            bg_secondary = self.bg_secondary,
            bg_tertiary = self.bg_tertiary,
            bg_hover = self.bg_hover,
            bg_selected = self.bg_selected,
            bg_input = self.bg_input,
            text_primary = self.text_primary,
            text_secondary = self.text_secondary,
            text_muted = self.text_muted,
            text_inverse = self.text_inverse,
            accent = self.accent,
            accent_hover = self.accent_hover,
            accent_dim = self.accent_dim,
            border = self.border,
            border_focus = self.border_focus,
            match_highlight = self.match_highlight,
            match_highlight_bg = self.match_highlight_bg,
            tag_bg = self.tag_bg,
            tag_text = self.tag_text,
            tag_border = self.tag_border,
            success = self.success,
            warning = self.warning,
            error = self.error,
            spacing_xs = self.spacing_xs,
            spacing_sm = self.spacing_sm,
            spacing_md = self.spacing_md,
            spacing_lg = self.spacing_lg,
            spacing_xl = self.spacing_xl,
            font_family = self.font_family,
            font_mono = self.font_mono,
            font_size_xs = self.font_size_xs,
            font_size_sm = self.font_size_sm,
            font_size_md = self.font_size_md,
            font_size_lg = self.font_size_lg,
            font_size_xl = self.font_size_xl,
            font_weight_normal = self.font_weight_normal,
            font_weight_medium = self.font_weight_medium,
            font_weight_bold = self.font_weight_bold,
            line_height = self.line_height,
            radius_sm = self.radius_sm,
            radius_md = self.radius_md,
            radius_lg = self.radius_lg,
            radius_full = self.radius_full,
            shadow_sm = self.shadow_sm,
            shadow_md = self.shadow_md,
            shadow_lg = self.shadow_lg,
            transition_fast = self.transition_fast,
            transition_normal = self.transition_normal,
            sidebar_width = self.sidebar_width,
            search_height = self.search_height,
            result_item_height = self.result_item_height,
            action_bar_height = self.action_bar_height,
        )
    }

    /// Generate the complete stylesheet (variables + component styles + custom CSS).
    pub fn to_stylesheet(&self) -> String {
        let mut css = self.to_css_variables();
        css.push_str("\n\n");
        css.push_str(COMPONENT_CSS);
        if !self.custom_css.is_empty() {
            css.push_str("\n\n/* Custom overrides */\n");
            css.push_str(&self.custom_css);
        }
        css
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

/// Component CSS that references theme variables.
/// Separated from the theme so consumers don't need to rewrite component styles.
const COMPONENT_CSS: &str = r#"
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  background-color: var(--bg-primary);
  color: var(--text-primary);
  font-family: var(--font-family);
  font-size: var(--font-size-md);
  line-height: var(--line-height);
}

/* ── Layout ─────────────────────────────────────────────── */

.launcher {
  display: flex;
  height: 100vh;
  overflow: hidden;
  background: var(--bg-primary);
  position: relative;
}

.launcher-sidebar {
  width: var(--sidebar-width);
  background: var(--bg-tertiary);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  overflow-y: auto;
  flex-shrink: 0;
}

.launcher-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
  overflow: hidden;
}

/* Palette mode: no sidebar, compact */
.launcher.palette-mode {
  height: auto;
  max-height: 460px;
}
.launcher.palette-mode .launcher-sidebar {
  display: none;
}

/* ── Search Bar ─────────────────────────────────────────── */

.search-container {
  display: flex;
  align-items: center;
  padding: var(--spacing-lg) var(--spacing-xl);
  border-bottom: 1px solid var(--border);
  gap: var(--spacing-md);
  height: var(--search-height);
  background: var(--bg-primary);
  flex-shrink: 0;
}

.search-icon {
  color: var(--accent);
  font-size: 18px;
  flex-shrink: 0;
  opacity: 0.8;
}

.search-input {
  flex: 1;
  background: transparent;
  border: none;
  outline: none;
  color: var(--text-primary);
  font-size: var(--font-size-lg);
  font-family: var(--font-family);
  caret-color: var(--accent);
}

.search-input::placeholder {
  color: var(--text-muted);
}

.search-meta {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  flex-shrink: 0;
}

.search-count {
  color: var(--text-muted);
  font-size: var(--font-size-sm);
}

.search-mode-toggle {
  padding: 2px 8px;
  border-radius: var(--radius-sm);
  background: var(--bg-secondary);
  color: var(--text-muted);
  font-size: var(--font-size-xs);
  cursor: pointer;
  border: 1px solid var(--border);
  transition: all var(--transition-fast);
  font-family: var(--font-family);
}
.search-mode-toggle:hover {
  color: var(--text-primary);
  border-color: var(--accent);
}

/* ── Filter Chips ───────────────────────────────────────── */

.filter-chips {
  display: flex;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-xl);
  border-bottom: 1px solid var(--border);
  overflow-x: auto;
  flex-shrink: 0;
  background: var(--bg-primary);
}
.filter-chips:empty {
  display: none;
}

.filter-chip {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: 3px 10px;
  border-radius: var(--radius-full);
  background: var(--bg-secondary);
  color: var(--text-secondary);
  font-size: var(--font-size-xs);
  cursor: pointer;
  border: 1px solid transparent;
  transition: all var(--transition-fast);
  white-space: nowrap;
  font-family: var(--font-family);
}
.filter-chip:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}
.filter-chip.active {
  background: var(--accent);
  color: var(--text-inverse);
  border-color: var(--accent);
}
.filter-chip.excluded {
  background: var(--error);
  color: var(--text-inverse);
  border-color: var(--error);
  opacity: 0.85;
}

.filter-chip-icon {
  font-size: 12px;
}
.filter-chip-clear {
  margin-left: 2px;
  opacity: 0.6;
  font-size: 10px;
}
.filter-chip-clear:hover {
  opacity: 1;
}

/* ── Results ────────────────────────────────────────────── */

.results {
  flex: 1;
  overflow-y: auto;
  padding: var(--spacing-sm) var(--spacing-md);
}

.results::-webkit-scrollbar { width: 6px; }
.results::-webkit-scrollbar-track { background: transparent; }
.results::-webkit-scrollbar-thumb {
  background: var(--bg-hover);
  border-radius: 3px;
}
.results::-webkit-scrollbar-thumb:hover {
  background: var(--bg-selected);
}

/* Section headers (provider groups, favorites/recents) */
.result-section {
  padding: var(--spacing-md) var(--spacing-md) var(--spacing-sm);
  font-size: var(--font-size-xs);
  font-weight: var(--font-weight-bold);
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

/* ── Result Item ────────────────────────────────────────── */

.result-item {
  display: flex;
  align-items: center;
  padding: var(--spacing-md) var(--spacing-lg);
  border-radius: var(--radius-md);
  cursor: pointer;
  gap: var(--spacing-lg);
  transition: background-color var(--transition-fast);
  min-height: var(--result-item-height);
}

.result-item:hover {
  background-color: var(--bg-hover);
}

.result-item.selected {
  background-color: var(--bg-selected);
}

.result-icon-wrap {
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-md);
  background: var(--bg-secondary);
  color: var(--accent);
  font-size: 16px;
  flex-shrink: 0;
  font-weight: var(--font-weight-bold);
}

.result-body {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.result-label {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.result-sub {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.result-meta {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  flex-shrink: 0;
}

.result-provider-badge {
  font-size: var(--font-size-xs);
  color: var(--text-muted);
  padding: 2px 6px;
  background: var(--bg-secondary);
  border-radius: var(--radius-sm);
}

.result-action-count {
  font-size: var(--font-size-xs);
  color: var(--text-muted);
  opacity: 0.6;
}

.result-shortcut {
  font-size: var(--font-size-xs);
  color: var(--text-muted);
  font-family: var(--font-mono);
  padding: 1px 5px;
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
}

/* ── Tag Pills ──────────────────────────────────────────── */

.tag-pills {
  display: flex;
  gap: var(--spacing-xs);
  flex-wrap: wrap;
  margin-top: 2px;
}

.tag-pill {
  font-size: var(--font-size-xs);
  padding: 0 6px;
  border-radius: var(--radius-full);
  background: var(--tag-bg);
  color: var(--tag-text);
  border: 1px solid var(--tag-border);
  white-space: nowrap;
  line-height: 1.6;
}

/* ── Match Highlight ────────────────────────────────────── */

.match-char {
  color: var(--match-highlight);
  font-weight: var(--font-weight-bold);
}

/* ── Sidebar ────────────────────────────────────────────── */

.sidebar-header {
  padding: var(--spacing-lg) var(--spacing-xl);
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-bold);
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  border-bottom: 1px solid var(--border);
}

.sidebar-section {
  padding: var(--spacing-sm) 0;
}

.sidebar-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  padding: var(--spacing-sm) var(--spacing-xl);
  color: var(--text-secondary);
  font-size: var(--font-size-sm);
  cursor: pointer;
  transition: all var(--transition-fast);
}
.sidebar-item:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}
.sidebar-item.active {
  background: var(--bg-selected);
  color: var(--accent);
  font-weight: var(--font-weight-medium);
}

.sidebar-item-icon {
  font-size: 14px;
  width: 18px;
  text-align: center;
  flex-shrink: 0;
}

.sidebar-item-label {
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.sidebar-item-count {
  font-size: var(--font-size-xs);
  color: var(--text-muted);
}

.sidebar-indent-1 { padding-left: calc(var(--spacing-xl) + 18px); }
.sidebar-indent-2 { padding-left: calc(var(--spacing-xl) + 36px); }

.sidebar-toggle {
  font-size: 10px;
  width: 18px;
  text-align: center;
  cursor: pointer;
  color: var(--text-muted);
  transition: transform var(--transition-fast);
}
.sidebar-toggle.collapsed {
  transform: rotate(-90deg);
}

/* ── Action Bar ─────────────────────────────────────────── */

.action-bar {
  display: flex;
  align-items: center;
  padding: var(--spacing-sm) var(--spacing-xl);
  border-top: 1px solid var(--border);
  height: var(--action-bar-height);
  background: var(--bg-tertiary);
  gap: var(--spacing-lg);
  flex-shrink: 0;
}

.action-bar-left {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  flex: 1;
  min-width: 0;
}

.action-bar-right {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  flex-shrink: 0;
}

.action-btn {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  font-size: var(--font-size-xs);
  color: var(--text-muted);
  cursor: pointer;
  padding: 2px 6px;
  border-radius: var(--radius-sm);
  transition: all var(--transition-fast);
  font-family: var(--font-family);
  border: none;
  background: none;
}
.action-btn:hover {
  color: var(--text-primary);
  background: var(--bg-hover);
}

.action-kbd {
  font-family: var(--font-mono);
  font-size: var(--font-size-xs);
  padding: 1px 4px;
  border-radius: var(--radius-sm);
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  color: var(--text-muted);
}

.action-divider {
  width: 1px;
  height: 16px;
  background: var(--border);
}

/* ── Empty State ────────────────────────────────────────── */

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 48px 24px;
  color: var(--text-muted);
  gap: var(--spacing-md);
}

.empty-icon {
  font-size: 36px;
  opacity: 0.4;
}

.empty-text {
  font-size: var(--font-size-md);
}

.empty-hint {
  font-size: var(--font-size-sm);
  opacity: 0.6;
}
"#;

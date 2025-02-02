use std::io::{stdout, Write};

use anyhow::Result;
use crossterm::{cursor::{MoveTo, RestorePosition, SavePosition, SetCursorStyle}, execute, queue, terminal::{Clear, ClearType}};

use crate::Term;

impl Term {
	#[inline]
	pub fn kill_to_end(stdout: &mut impl Write) -> Result<()> {
		Ok(queue!(stdout, Clear(ClearType::UntilNewLine))?)
	}

	#[inline]
	pub fn move_to(stdout: &mut impl Write, x: u16, y: u16) -> Result<()> {
		Ok(queue!(stdout, MoveTo(x, y))?)
	}

	#[inline]
	pub fn move_lock<W, F>(mut stdout: W, (x, y): (u16, u16), cb: F) -> Result<()>
	where
		W: Write,
		F: FnOnce(&mut W) -> Result<()>,
	{
		#[cfg(target_os = "windows")]
		{
			use crossterm::cursor::{Hide, Show};
			queue!(&mut stdout, SavePosition, MoveTo(x, y), Show)?;
			let result = cb(&mut stdout);
			queue!(&mut stdout, Hide, RestorePosition)?;
			stdout.flush()?;
			result
		}
		#[cfg(not(target_os = "windows"))]
		{
			queue!(&mut stdout, SavePosition, MoveTo(x, y))?;
			let result = cb(&mut stdout);
			queue!(&mut stdout, RestorePosition)?;
			stdout.flush()?;
			result
		}
	}

	#[inline]
	pub fn set_cursor_block() -> Result<()> { Ok(execute!(stdout(), SetCursorStyle::BlinkingBlock)?) }

	#[inline]
	pub fn set_cursor_bar() -> Result<()> { Ok(execute!(stdout(), SetCursorStyle::BlinkingBar)?) }
}

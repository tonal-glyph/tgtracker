//@ The FT2 edit mode is for the die-hard FT2 users and probably isn't very intuitive to beginners. Please note that not all FT2 shortcuts are implemented yet and some may differ for various technical reasons. Also note that this edit mode may not be optimal on Pocket PC because of the limitations of some input devices.
//@
//@ ## Section switching
//@
//@ **Ctrl-**
//@ - **A**	Advanced edit
//@ - **C**	Configuration
//@ - **D**	Disk operations
//@ - **I**	Instrument editor
//@ - **R**	Disk recorder
//@ - **S**	Sample editor
//@ - **T**	Transpose
//@ - **X**	Main screen
//@ - **Z**	Toggle scopes
//@
//@ ## Global
//@
//@ - **2, 3, 5, 6…**	Play / insert notes (depending on whether edit mode is on)
//@ - **Q, W, E, R…**
//@ - **S, D, F, G…**
//@ - **Z, X, C, V…**
//@ - **F1…F8**	Select octave
//@ - **Right Ctrl**	Play song from current order
//@ - **Enter**	Play song from current order
//@ - **Right Alt**	Play current pattern from beginning (Windows &SDL)
//@ - **Ctrl-Enter**	Play current pattern from beginning
//@ - **Shift-Enter**	Play current pattern from current row
//@ - **Shift-F9**	Play current pattern from beginning (same as Ctrl-Enter/Right Alt)
//@ - **Shift-F10**	Play current pattern from position after the first quarter of the pattern length
//@ - **Shift-F11**	Play current pattern from position after the second quarter of the pattern length
//@ - **Shift-F12**	Play current pattern from position after the third quarter of the pattern length
//@ - **Alt-Space**	Play song from current row (stop and return when keys are released)
//@ - **Shift-Space**	Play row by row
//@ - **Space**	Stop / Edit
//@ - **Shift-Left**	Increase song position
//@ - **Shift-Right**	Decrease song position
//@ - **Ctrl-Left**	Increase current pattern number
//@ - **Ctrl-Right**	Decrease current pattern number
//@ - **Ctrl-F9**	Delete current order position
//@ - **Ctrl-F10**	Insert new order position
//@ - **Ctrl-F11**	Decrease current order pattern number
//@ - **Ctrl-F12**	Increase current order pattern number
//@ - **Key below ESC** (ANSI: Alt-Minus)*	Increase Add value
//@ - **Shift-key below ESC** (ANSI: Alt-Plus)*	Decrease Add value
//@ - **Ctrl-F**	Toggle song follow
//@ - **Ctrl-P**	Toggle prospective pattern view
//@ - **Ctrl-W**	Toggle pattern wrapping
//@ - **Ctrl-L**	Toggle pattern change behavior (live mode)
//@ - **Shift-Ctrl-L**	Load song
//@ - **Shift-R**	Toggle record mode
//@ - **Shift-Ctrl-S**	Save song
//@ - **Esc**	Exit program
//@ - * Please note in this table, "Key under esc" refers to the tilde / tick key, section symbol / plusminus key or the ring accent / circumflex key depending on your ISO keyboard, but does not exist on ANSI layouts. See: this issue for details
//@
//@ ## Pattern editor
//@
//@ - **Cursor keys**	Move around
//@ - **PageUp**	Jump 16 rows up
//@ - **PageDown**	Jump 16 rows down
//@ - **Home**	Jump to first row
//@ - **End**	Jump to last row
//@ - **Tab**	Jump to next track
//@ - **Shift-Tab**	Jump to previous track
//@ - **Alt-Q…I**	Jump to track (0…7) MOD N-Channels
//@ - **Alt-A…K**	Jump to track (8…15) MOD N-Channels
//@ - **F9**	Jump to beginning of the pattern
//@ - **F10**	Jump to position ¼ through the pattern
//@ - **F11**	Jump to position halfway through the pattern
//@ - **F12**	Jump to position ¾ through the pattern
//@ - **The key right of LShift**	Enter key-off
//@ - **Caps-Lock**	Enter key-off (Windows only)
//@ - **1**	Enter key-off (OS X only)
//@ - **Del**	Delete note or volume column at cursor
//@ - **Shift-Del**	Delete note, volume and effect at cursor
//@ - **Ctrl-Del**	Delete volume and effect at cursor
//@ - **Alt-Delete**	Delete effect at cursor
//@ - **Ins**	Insert space on current track at cursor position (F13 on mac)
//@ - **Shift-Ins**	Insert row at cursor position (shift-F13 on mac)
//@ - **Backspace**	Delete previous note
//@ - **Shift-Backspace**	Delete previous row
//@
//@ ## Clipboard operations
//@
//@ - **Alt-Cursor** keys	Select block
//@ - **Shift-Alt-Cursor** keys	Extend block
//@ - **Alt-F3**	Cut block
//@ - **Alt-F4**	Copy block (yes, even under Windows =)
//@ - **Alt-F5**	Paste block
//@ - **Alt-F6**	Porous paste block
//@ - **Shift-F3**	Cut track
//@ - **Shift-F4**	Copy track
//@ - **Shift-F5**	Paste track
//@ - **Shift-F6**	Porous paste track
//@ - **Ctrl-F3**	Cut pattern
//@ - **Ctrl-F4**	Copy pattern
//@ - **Ctrl-F5**	Paste pattern
//@ - **Ctrl-F6**	Porous paste pattern
//@
//@ ## Additional shortcuts (not found in FT2)
//@
//@ - **Ctrl-Alt-Z**	Undo
//@ - **Ctrl-Alt-Y**	Redo
//@ - **Ctrl-Alt-A**	Select entire pattern
//@ - **Shift-I**	Interpolate values
//@
//@ ## Volume scaling
//@
//@ - **Alt-V**	Volume scale block
//@ - **Shift-V**	Volume scale track
//@ - **Ctrl-V**	Volume scale pattern
//@
//@ ## Command/Volume macro
//@
//@ - **Shift-Alt-1…0**	Read command/volume at cursor
//@ - **Alt-1…0**	Write command/volume at cursor
//@
//@ ## Transpose
//@
//@ - **Alt-F7**	Transpose current instrument in block down
//@ - **Alt-F8**	Transpose current instrument in block up
//@ - **Shift-F7**	Transpose current instrument in track down
//@ - **Shift-F8**	Transpose current instrument in track up
//@ - **Ctrl-F7**	Transpose current instrument in pattern down
//@ - **Ctrl-F8**	Transpose current instrument in pattern up
//@ - **Alt-F1**	Transpose all instruments in block down
//@ - **Alt-F2**	Transpose all instruments in block up
//@ - **Shift-F1**	Transpose all instruments in track down
//@ - **Shift-F2**	Transpose all instruments in track up
//@ - **Ctrl-F1**	Transpose all instruments in pattern down
//@ - **Ctrl-F2**	Transpose all instruments in pattern up
//@
//@ ## Instrument selection
//@
//@ - **Shift-Up**	Select previous instrument
//@ - **Shift-Down**	Select next instrument
//@ - **Ctrl-Shift-Up**	Select previous sample
//@ - **Ctrl-Shift-Down**	Select next sample
//@ - You can also quick-type the hex-number of the instrument you want to select on the numeric keypad, the layout is like this:
//@
//@ PC	Mac
//@ Num 0…9	Num 0…9	Digit 0…9
//@ Num /	Num =	Digit A
//@ Num *	Num /	Digit B
//@ Num -	Num *	Digit C
//@ Num +	Num -	Digit D
//@ Num Enter	Num +	Digit E
//@ Num ,	Num Enter	Digit F
//@
//@ ## Sample editor
//@
//@ - **Shift & drag**	Quick draw
//@ - **Ctrl & drag**	Resize selection
//@ - **Alt & drag**	Move selection or loop range

<script lang="ts">
	import { goto } from '$app/navigation';
	import { boxes } from '$lib/boxes';
	import {
		PaintBucket,
		Pipette,
		Plus,
		Download,
		Upload,
		Brush,
		SquareDashed,
		List,
		Eraser,
		Pen,
		MousePointer2,
		Home,
		SquareDashedMousePointer,
		ClipboardPaste,
		ClipboardCopy,
		Trash2,
		Scissors,
		OctagonX
	} from 'lucide-svelte';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { toast } from 'svelte-sonner';
	import { Popover, Label } from 'bits-ui';
	import { special_characters } from '$lib/special_characters';
	import { flyAndScale } from '$lib/fly_and_scale';
	import TooltipButton from '$lib/tooltip_button.svelte';
	import Dialog from '$lib/dialog.svelte';

	let width: number;
	let height: number;
	let tiles: Array<Array<string>> = [];

	$: {
		width = parseInt($page.url.searchParams.get('width') || '30');
		height = parseInt($page.url.searchParams.get('height') || '18');
		clear_board();

		let load = $page.url.searchParams.get('load');
		if (load) {
			let lines = load.split('\n');

			let x = 0;
			for (let line of lines) {
				for (let y = 0; y < line.length; y++) {
					set(x, y, line[y]);
				}
				x += 1;
			}
		}
	}

	let selected: Array<number> | null = null;
	let clipboard: string = ' ';
	let character: string = '-';
	let character_input_selected: boolean = false;
	let selected_tool: Tool = 'cursor';
	type Tool = 'brush' | 'cursor' | 'eraser' | 'select' | 'eyedropper' | 'sketch';
	let is_mouse_down = false;
	let selection: null | Array<Array<number>> = null;
	let hovered: Array<number> = [999, 999];
	let is_selecting = false;
	let char_popover_open = false;
	let box_popover_open = false;
	let export_string = '';
	let show_export_popup = false;
	let show_import_popup = false;
	let show_new_popup = false;
	let width_input = 30,
		height_input = 18;
	let load_input = '';
	let sketch_start_pos: number[] | null = [999, 999];

	function clear_board() {
		tiles = Array(height)
			.fill(null)
			.map(() => Array(width).fill(' '));
	}

	function click(x: number, y: number) {
		character_input_selected = false;
		is_mouse_down = true;

		console.log(x, y);

		if (selected_tool == 'cursor') {
			if (selected === null || selected[0] != x || selected[1] != y) {
				selected = [x, y];
			} else {
				selected = null;
			}
		} else if (selected_tool == 'brush') {
			selected = null;
			set(x, y, character);
		} else if (selected_tool == 'eraser') {
			selected = null;
			set(x, y, ' ');
		} else if (selected_tool == 'select') {
			if (!is_selecting) {
				selection = [
					[x, y],
					[x, y]
				];
				is_selecting = true;
			} else {
				selection[1] = [x, y];
			}
			console.log(selection);
		} else if (selected_tool == 'eyedropper') {
			character = get(x, y);
		} else if (selected_tool == 'sketch') {
			if (!sketch_start_pos) {
				sketch_start_pos = [x, y];
			}
		}
	}

	function open_export_popup() {
		export_string = export_selection();
		show_export_popup = true;
	}

	function box_selection(hl: string, vl: string, tr: string, tl: string, br: string, bl: string) {
		if (!selection) return;

		const [[startX, startY], [endX, endY]] = selection;
		const minX = Math.min(startX, endX);
		const maxX = Math.max(startX, endX);
		const minY = Math.min(startY, endY);
		const maxY = Math.max(startY, endY);

		set(minX, minY, tl);
		set(minX, maxY, tr);
		set(maxX, minY, bl);
		set(maxX, maxY, br);

		for (let x = minX + 1; x < maxX; x++) {
			set(x, minY, vl);
			set(x, maxY, vl);
		}

		for (let y = minY + 1; y < maxY; y++) {
			set(minX, y, hl);
			set(maxX, y, hl);
		}
	}

	function get(x, y) {
		return tiles[x][y];
	}

	function set(x, y, value) {
		if (x < 0 || x >= height || y < 0 || y >= width) return;
		tiles[x][y] = value;
	}

	let select_clipboard: string[][] = [['']];

	function export_selection(): string {
		if (selection) {
			const [[startX, startY], [endX, endY]] = selection;
			const minX = Math.min(startX, endX);
			const maxX = Math.max(startX, endX);
			const minY = Math.min(startY, endY);
			const maxY = Math.max(startY, endY);

			let string = '';
			for (let x = minX; x <= maxX; x++) {
				let row = '';
				for (let y = minY; y <= maxY; y++) {
					row += get(x, y);
				}
				string += row + '\n';
			}

			return string;
		}

		return '';
	}

	function copy_selection() {
		if (selection) {
			const [[startX, startY], [endX, endY]] = selection;
			const minX = Math.min(startX, endX);
			const maxX = Math.max(startX, endX);
			const minY = Math.min(startY, endY);
			const maxY = Math.max(startY, endY);

			let to_copy: string[][] = [];
			for (let x = minX; x <= maxX; x++) {
				let row: string[] = [];
				for (let y = minY; y <= maxY; y++) {
					row.push(get(x, y));
				}
				to_copy.push(row);
			}

			select_clipboard = to_copy;
			toast.success('Copied selection.');
			console.log(to_copy);
			return;
		}

		if (selected) {
			clipboard = get(selected[0], selected[1]);
			toast.success('Copied character.');
		}
	}

	function cut_selection() {
		if (selection) {
			copy_selection();
			const [[startX, startY], [endX, endY]] = selection;
			const minX = Math.min(startX, endX);
			const maxX = Math.max(startX, endX);
			const minY = Math.min(startY, endY);
			const maxY = Math.max(startY, endY);

			for (let x = minX; x <= maxX; x++) {
				for (let y = minY; y <= maxY; y++) {
					set(x, y, ' ');
				}
			}

			toast.success('Cut selection.');
			selection = null;
			is_selecting = false;
		}
	}

	function fill_selection(char: string, action: string = 'Filled') {
		if (selection) {
			const [[startX, startY], [endX, endY]] = selection;
			const minX = Math.min(startX, endX);
			const maxX = Math.max(startX, endX);
			const minY = Math.min(startY, endY);
			const maxY = Math.max(startY, endY);

			for (let x = minX; x <= maxX; x++) {
				for (let y = minY; y <= maxY; y++) {
					set(x, y, char);
				}
			}

			toast.success(action + ' selection.');
			selection = null;
			is_selecting = false;
		} else if (selected) {
			set(selected[0], selected[1], char);
		}
	}

	function clear_selection() {
		fill_selection(' ', 'Cleared');
	}

	function paste_selection() {
		if (selected_tool == 'cursor' && selected) {
			set(selected[0], selected[1], clipboard);
			toast.success('Pasted character.');
		} else if (selected_tool == 'select') {
			let x, y;
			if (hovered[0] == 999 && hovered[1] == 999) {
				if (selected) {
					[x, y] = selected;
				} else if (selection) {
					[x, y] = selection[0];
				} else {
					[x, y] = hovered;
				}
			} else {
				[x, y] = hovered;
			}
			console.log(x, y);
			for (let i = 0; i < select_clipboard.length; i++) {
				for (let j = 0; j < select_clipboard[i].length; j++) {
					if (x + i < height && y + j < width) {
						set(x + i, y + j, select_clipboard[i][j]);
					}
				}
			}
			toast.success('Pasted selection.');
		}
	}

	function isInSelection(x: number, y: number): boolean {
		if (!selection) return false;
		const [[startX, startY], [endX, endY]] = selection;
		const minX = Math.min(startX, endX);
		const maxX = Math.max(startX, endX);
		const minY = Math.min(startY, endY);
		const maxY = Math.max(startY, endY);
		return x >= minX && x <= maxX && y >= minY && y <= maxY;
	}

	function change_tool(tool: Tool) {
		selected_tool = tool;
		selected = null;
		character_input_selected = false;
		if (tool !== 'select') {
			selection = null;
			is_selecting = false;
		}
	}

	onMount(() => {
		addEventListener('mousedown', (event) => {
			if (event.button === 0) {
				is_mouse_down = true;
			}
		});
		addEventListener('mouseup', (event) => {
			is_mouse_down = false;

			if (sketch_start_pos) {
				let [hy, hx] = hovered;
				let [sy, sx] = sketch_start_pos;

				if (hx != sx || hy != sy) {
					if (hy == sy) {
						set(sy, sx, '-');
					} else if (hx == sx) {
						set(sy, sx, '|');
					} else if ((sx < hx && sy > hy) || (sx > hx && sy < hy)) {
						set(sy, sx, '/');
					} else {
						set(sy, sx, '\\');
					}
				}
			}

			sketch_start_pos = null;
			if (selected_tool === 'select' && is_selecting) {
				is_selecting = false;
			}
		});
		addEventListener('keydown', (event) => {
			console.log(event.key);

			if (show_export_popup || show_import_popup || show_new_popup) {
				return;
			}

			if (event.key == 'Escape') {
				event.preventDefault();
				selected = null;
				character_input_selected = false;
				selection = null;
				is_selecting = false;
				return;
			}

			if (event.shiftKey && event.ctrlKey && event.key == 'C') {
				event.preventDefault();
				open_export_popup();
				return;
			}

			if (event.ctrlKey) {
				if (event.key === 'b') {
					change_tool('brush');
					event.preventDefault();
					return;
				} else if (event.key === 'd') {
					change_tool('cursor');
					event.preventDefault();
					return;
				} else if (event.key === 'f') {
					character_input_selected = !character_input_selected;
					event.preventDefault();
					return;
				} else if (event.key === 'e') {
					change_tool('eraser');
					event.preventDefault();
					return;
				} else if (event.key === 'r') {
					change_tool('select');
					event.preventDefault();
					return;
				} else if (event.key === 'g') {
					clear_board();
					event.preventDefault();
					return;
				} else if (event.key === 'j') {
					if (selection) {
						box_popover_open = true;
					}
					event.preventDefault();
					return;
				} else if (event.key === 'x') {
					event.preventDefault();
					cut_selection();
					return;
				} else if (event.key === 'y') {
					event.preventDefault();
					fill_selection(character);
					return;
				} else if (event.key === 'i') {
					event.preventDefault();
					change_tool('eyedropper');
					return;
				}
			}

			if (selected_tool == 'cursor' && selected) {
				switch (event.key) {
					case 'Backspace': {
						event.preventDefault();
						selected[1]--;
						set(selected[0], selected[1], ' ');
						return;
					}
					case 'ArrowLeft': {
						event.preventDefault();
						selected[1]--;
						return;
					}
					case 'ArrowRight': {
						event.preventDefault();
						selected[1]++;
						return;
					}
					case 'ArrowUp': {
						event.preventDefault();
						selected[0]--;
						return;
					}
					case 'ArrowDown': {
						event.preventDefault();
						selected[0]++;
						return;
					}
				}
			}

			if (
				((selected_tool == 'select' && selection) || (selected_tool == 'cursor' && selected)) &&
				event.key == 'Delete'
			) {
				clear_selection();
			}

			if (character_input_selected && event.key.length === 1) {
				character = event.key;
				character_input_selected = false;
				return;
			}

			if (selected_tool == 'select') {
				if (selection && event.ctrlKey) {
					if (event.key === 'c') {
						event.preventDefault();
						copy_selection();
						return;
					} else if (event.key === 'x') {
						event.preventDefault();
						cut_selection();
						return;
					}
				}
				if (event.ctrlKey) {
					if (event.key === 'v') {
						event.preventDefault();
						paste_selection();
						return;
					}
				}
			}

			if (selected_tool == 'cursor') {
				if (selected && event.ctrlKey) {
					if (event.key === 'c') {
						event.preventDefault();
						copy_selection();
						return;
					} else if (event.key === 'v') {
						event.preventDefault();
						paste_selection();
						return;
					}
				}

				if (selected && event.key.length === 1) {
					event.preventDefault();
					const [x, y] = selected;
					set(x, y, event.key);
					selected[1]++;
					return;
				}
			}
		});
	});
</script>

<TooltipButton
	onClick={() => {
		goto('/');
	}}
	title="Back to home"
	className="top-4 left-4 fixed"
>
	<Home />
</TooltipButton>

<div class="fixed right-4 top-4">
	<TooltipButton
		onClick={() => {
			show_new_popup = true;
		}}
		tooltip="New drawing (Ctrl+Shift+N)"
	>
		<Plus />
	</TooltipButton>

	<TooltipButton
		onClick={() => {
			show_import_popup = true;
		}}
		tooltip="Import drawing (Ctrl+Shift+I)"
	>
		<Download />
	</TooltipButton>

	<TooltipButton
		onClick={() => {
			open_export_popup();
		}}
		disabled={!selection}
		tooltip="Export selection (Ctrl+Shift+C)"
	>
		<Upload />
	</TooltipButton>
</div>

<div class="flex h-[100vh] w-[100vw] items-center {selected_tool} cursor-this">
	<div
		class="h-[{60 * height}] w-[{width * 36}px]
    mx-auto my-auto w-fit overflow-auto rounded-xl ring ring-2 ring-neutral-200"
	>
		{#each tiles as row, x}
			<div class="flex">
				{#each row as tile, y}
					<button
						on:mouseenter={() => {
							hovered = [x, y];
							if (is_mouse_down) {
								click(x, y);
							}
						}}
						on:mouseleave={() => {
							hovered = [999, 999];
						}}
						on:mousedown={(event) => {
							if (event.button === 0) {
								click(x, y);
							}
						}}
						class="my-auto h-full w-full text-center text-6xl hover:bg-neutral-200
          {selected && x === selected[0] && y === selected[1]
							? 'bg-blue-400 hover:!bg-blue-300'
							: x == hovered[0] || y == hovered[1]
								? 'bg-neutral-200/50'
								: 'bg-neutral-100'}
          {isInSelection(x, y) ? '!bg-blue-300/30 hover:!bg-blue-300/40' : ''}"
					>
						<pre>{tile}</pre>
					</button>
				{/each}
			</div>
		{/each}
	</div>
</div>

<div class="cursor-this fixed bottom-0 left-0 z-10 flex w-full items-center gap-1 bg-white p-8">
	<TooltipButton
		noClass
		className="pointer rounded-2xl p-3 text-[24px] mr-3
    {character_input_selected ? '!bg-blue-300' : '!bg-neutral-200'}"
		onClick={() => {
			character_input_selected = !character_input_selected;
		}}
		tooltip="Change character. (Ctrl+F)"
	>
		<pre>{character}</pre>
	</TooltipButton>

	{#each [['cursor', MousePointer2, 'Ctrl+D'], ['brush', Brush, 'Ctrl+B'], ['eraser', Eraser, 'Ctrl+E'], ['select', SquareDashedMousePointer, 'Ctrl+R'], ['eyedropper', Pipette, 'Ctrl+I'], ['sketch', Pen, 'Ctrl+S']] as [tool, icon, keybind]}
		<TooltipButton
			onClick={() => {
				change_tool(tool);
			}}
			tooltip="{tool} ({keybind})"
			className={selected_tool === tool ? 'bg-neutral-200' : 'hover:bg-neutral-100'}
		>
			<svelte:component this={icon} />
		</TooltipButton>
	{/each}

	<div class="flex-1"></div>

	{#if selected_tool == 'select' || selected_tool == 'cursor'}
		<TooltipButton
			onClick={() => {
				copy_selection();
			}}
			disabled={!selection && !selected}
			tooltip="Copy selection (Ctrl+C)"
		>
			<ClipboardCopy />
		</TooltipButton>

		<TooltipButton
			onClick={() => {
				paste_selection();
			}}
			tooltip="Paste (Ctrl+V)"
		>
			<ClipboardPaste />
		</TooltipButton>

		<TooltipButton
			onClick={() => {
				clear_selection();
			}}
			disabled={!selection && !selected}
			tooltip="Clear selection (Delete)"
		>
			<OctagonX />
		</TooltipButton>
	{/if}

	{#if selected_tool == 'select'}
		<TooltipButton
			onClick={() => {
				cut_selection();
			}}
			disabled={!selection}
			tooltip="Cut selection (Ctrl+X)"
		>
			<Scissors />
		</TooltipButton>

		<TooltipButton
			onClick={() => {
				fill_selection(character);
			}}
			disabled={!selection}
			tooltip="Fill selection (Ctrl+Y)"
		>
			<PaintBucket />
		</TooltipButton>

		<Popover.Root bind:open={box_popover_open}>
			<Popover.Trigger>
				<TooltipButton disabled={!selection} tooltip="Box selection (Ctrl+J)">
					<SquareDashed />
				</TooltipButton>
			</Popover.Trigger>
			<Popover.Content
				transition={flyAndScale}
				class="cursor-this z-30 h-full max-h-[400px] w-full max-w-[500px] overflow-auto rounded-2xl border border-neutral-200 bg-neutral-100 p-4 shadow"
			>
				<div class="grid grid-cols-3">
					{#each boxes as { chars, name, preview }}
						<button
							on:click={() => {
								box_selection(chars.hl, chars.vl, chars.tr, chars.tl, chars.br, chars.bl);
								box_popover_open = false;
							}}
							class="pointer aspect-square rounded-xl p-1 text-center hover:bg-neutral-200/50"
						>
							<div class="text-center">{name}</div>
							<pre class="leading-none">{preview}</pre>
						</button>
					{/each}
				</div>
			</Popover.Content>
		</Popover.Root>
	{/if}

	<TooltipButton
		onClick={() => {
			clear_board();
		}}
		tooltip="Clear drawing (Ctrl+G)"
	>
		<Trash2 />
	</TooltipButton>

	<Popover.Root bind:open={char_popover_open}>
		<Popover.Trigger>
			<TooltipButton tooltip="Special characters">
				<List />
			</TooltipButton>
		</Popover.Trigger>
		<Popover.Content
			transition={flyAndScale}
			class="cursor-this z-30 h-full max-h-[400px] w-full max-w-[500px] overflow-auto rounded-2xl border border-neutral-200 bg-neutral-100 p-4 shadow"
		>
			<div class="grid grid-cols-12">
				{#each special_characters as char}
					<button
						on:click={() => {
							character = char;
							char_popover_open = false;
						}}
						class="pointer aspect-square rounded-xl p-1 text-center hover:bg-neutral-200/50"
					>
						{char}
					</button>
				{/each}
			</div>
		</Popover.Content>
	</Popover.Root>
</div>

<Dialog
	title="Export drawing"
	bind:open={show_export_popup}
	closeButtonText="Copy"
	closeButtonOnClick={() => {
		navigator.clipboard.writeText(export_string);
		toast.success('Copied to clipboard.');
	}}
>
	<p class="text-xs text-neutral-700">
		If you're putting this into Google Docs or some other office program, it will probably look
		weird. Make sure the text is in a monospaced font, and the line-height is set to 1, or 'single'.
	</p>
	<br />
	<pre
		class="mx-auto w-fit rounded-2xl bg-neutral-200 p-4 text-xl leading-none">{export_string}</pre>
	<br />
</Dialog>

<Dialog
	title="New drawing"
	bind:open={show_new_popup}
	closeButtonText="New"
	closeButtonOnClick={() => {
		window.location.href = '/draw?width=' + width_input + '&height=' + height_input;
	}}
>
	<div class="flex">
		<span class="pr-3">Width: </span><input
			class="flex-1 bg-neutral-100 outline-none"
			bind:value={width_input}
		/>
	</div>

	<div class="flex">
		<span class="pr-3">Height: </span><input
			class="flex-1 bg-neutral-100 outline-none"
			bind:value={height_input}
		/>
	</div>
</Dialog>

<Dialog
	title="Import drawing"
	bind:open={show_import_popup}
	closeButtonText="Load"
	closeButtonOnClick={() => {
		load_input = load_input.replace('\t', '    ');
		let width = load_input
			.split('\n')
			.map((line) => line.length)
			.sort((a, b) => b - a)[0];
		window.location.href =
			'/draw?load=' +
			encodeURIComponent(load_input) +
			'&width=' +
			width +
			'&height=' +
			load_input.split('\n').length;
	}}
>
	<div>
		<Label.Root for="load-textarea" class="pr-3">Load:</Label.Root>
		<br />
		<br />
		<textarea
			id="load-textarea"
			class="w-full flex-1 rounded-2xl border border-neutral-200 bg-neutral-100 p-4 outline-neutral-300"
			rows="8"
			bind:value={load_input}
		/>

		<br />
		<br />
	</div>
</Dialog>

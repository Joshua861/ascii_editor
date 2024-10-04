<script lang="ts">
	import {
		Brush,
		Eraser,
		MousePointer2,
		SquareDashedMousePointer,
		ClipboardPaste,
		ClipboardCopy,
		Trash2
	} from 'lucide-svelte';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { toast } from 'svelte-sonner';

	let width: number;
	let height: number;
	let tiles: Array<Array<string>> = [];

	$: {
		width = parseInt($page.url.searchParams.get('width') || '30');
		height = parseInt($page.url.searchParams.get('height') || '18');
		clear_board();
	}

	let selected: Array<number> | null = null;
	let clipboard: string | null = null;
	let character: string = '-';
	let character_input_selected: boolean = false;
	let selected_tool: Tool = 'cursor';
	type Tool = 'brush' | 'cursor' | 'eraser' | 'select';
	let is_mouse_down = false;
	let selection: null | Array<Array<number>> = null;
	let hovered: Array<number> = [999, 999];
	let is_selecting = false;

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
		}
	}

	function get(x, y) {
		return tiles[x][y];
	}

	function set(x, y, value) {
		if (x < 0 || x >= height || y < 0 || y >= width) return;
		tiles[x][y] = value;
	}

	function copy_selection() {
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
			navigator.clipboard.writeText(string);
			toast.success('Copied selection to clipboard');
			return;
		}

		if (selected) {
			clipboard = get(selected[0], selected[1]);
		}
	}

	function paste() {
		set(selected[0], selected[1], clipboard);
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
			if (selected_tool === 'select' && is_selecting) {
				is_selecting = false;
			}
		});
		addEventListener('keydown', (event) => {
			console.log(event.key);

			if (event.key == 'Escape') {
				event.preventDefault();
				selected = null;
				character_input_selected = false;
				selection = null;
				is_selecting = false;
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

			if (character_input_selected && event.key.length === 1) {
				character = event.key;
				character_input_selected = false;
				return;
			}

			if (selected_tool == 'select') {
				if (selection && event.key === 'c' && event.ctrlKey) {
					event.preventDefault();
					copy_selection();
					return;
				}
			}

			if (selected_tool == 'cursor') {
				if (selected && event.ctrlKey && event.key === 'c') {
					event.preventDefault();
					copy_selection();
					return;
				}

				if (selected && event.ctrlKey && event.key === 'v') {
					event.preventDefault();
					paste();
					return;
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

<div class="flex h-[100vh] w-[100vw] items-center {selected_tool} pointer-this">
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
							hovered = [-1, -1];
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

<div class="fixed bottom-0 left-0 z-10 flex w-full items-center gap-4 bg-white p-8">
	<button
		class="pointer rounded-2xl p-3 text-[24px]
    {character_input_selected ? 'bg-blue-300' : 'bg-neutral-200'}"
		on:click|preventDefault={() => {
			character_input_selected = !character_input_selected;
		}}
		title="Change character. (Ctrl+F)"
	>
		<pre>{character}</pre>
	</button>

	{#each [['cursor', MousePointer2, 'Ctrl+D'], ['brush', Brush, 'Ctrl+B'], ['eraser', Eraser, 'Ctrl+E'], ['select', SquareDashedMousePointer, 'Ctrl+R']] as [tool, icon, keybind]}
		<button
			on:click|preventDefault={() => {
				change_tool(tool);
			}}
			title="{tool} ({keybind})"
			class="pointer my-auto rounded-2xl p-4 {selected_tool === tool
				? 'bg-neutral-200'
				: 'hover:bg-neutral-100'}"
		>
			<svelte:component this={icon} />
		</button>
	{/each}

	<div class="flex-1"></div>

	{#if selected_tool == 'select' || selected_tool == 'cursor'}
		<button
			on:click|preventDefault={() => {
				if (selection) {
					copy_selection();
				}
			}}
			disabled={!selection || !selected}
			title="Copy selection (Ctrl+C)"
			class="disabled:hover-bg-white my-auto rounded-2xl p-4 hover:bg-neutral-100 disabled:cursor-not-allowed disabled:opacity-50"
		>
			<ClipboardCopy />
		</button>
	{/if}

	{#if selected_tool == 'cursor'}
		<button
			on:click|preventDefault={() => {
				if (selection) {
					paste();
				}
			}}
			disabled={!selected}
			title="Paste (Ctrl+V)"
			class="disabled:hover-bg-white my-auto rounded-2xl p-4 hover:bg-neutral-100 disabled:cursor-not-allowed disabled:opacity-50"
		>
			<ClipboardPaste />
		</button>
	{/if}

	<button
		on:click|preventDefault={() => {
			clear_board();
		}}
		title="Clear drawing (Ctrl+G)"
		class="disabled:hover-bg-white my-auto rounded-2xl p-4 hover:bg-neutral-100 disabled:cursor-not-allowed disabled:opacity-50"
	>
		<Trash2 />
	</button>
</div>

<style>
	.cursor * {
		cursor: url('/cursor.png'), auto !important;
	}
	.brush * {
		cursor: url('/brush.png'), auto !important;
	}
	.eraser * {
		cursor: url('/eraser.png'), auto !important;
	}
	.select * {
		cursor: url('/cursor.png'), auto !important;
	}
	.pointer {
		cursor: url('/pointer.png'), auto !important;
	}
	.pointer-this {
		cursor: url('/cursor.png'), auto !important;
	}
</style>

<script lang="ts">
	import { onMount } from 'svelte';
	import init, { solve_puzzle } from '../build/sudoku_solver.js';

	let input: string[][] = Array.from({ length: 9 }, () => new Array(9).fill(''));
	let focusedRow: number = 0;
	let focusedCol: number = 0;
	let wasmInitialized = false;

	onMount(async () => {
        await init();
        wasmInitialized = true;
    });

	const reset = () => {
		input = Array.from({ length: 9 }, () => new Array(9).fill(''));
	};

	const submit = () => {
		if (!wasmInitialized) {
            console.error("WASM module not initialized yet.");
            return;
        }

		const puzzle_flat = input.flat().map(Number);
		init().then(() => {
			const sol = solve_puzzle(puzzle_flat as unknown as Uint8Array);
			input = Array.from({ length: 9 }, (_, i) =>
				sol.slice(i * 9, i * 9 + 9)
			) as unknown as string[][];
		});
	};

	const handleInput = (event: Event, rowIndex: number, colIndex: number) => {
		const target = event.target as HTMLInputElement;
		let value = target.value;
		if (value.length > 1) {
			value = value.slice(-1);
		}
		if (!/^\d$/.test(value)) {
			value = '';
		}
		input[rowIndex][colIndex] = value;
	};

	const handleKeydown = (event: KeyboardEvent) => {
		switch (event.key) {
			case 'ArrowUp':
				focusedRow = Math.max(0, focusedRow - 1);
				break;
			case 'ArrowDown':
				focusedRow = Math.min(8, focusedRow + 1);
				break;
			case 'ArrowLeft':
				focusedCol = Math.max(0, focusedCol - 1);
				break;
			case 'ArrowRight':
				focusedCol = Math.min(8, focusedCol + 1);
				break;
			default:
				return;
		}
		focusCell(focusedRow, focusedCol);
	};

	const focusCell = (row: number, col: number) => {
		const inputElements = document.querySelectorAll('input');
		const index = row * 9 + col;
		const target = inputElements[index] as HTMLInputElement;
		target.focus();
	};
</script>

<div class="outline">
	<table>
		{#each input as row, rowIndex}
			<tr class={rowIndex % 3 == 2 ? 'h-div' : ''}>
				{#each row as _, colIndex}
					<td class={colIndex % 3 == 2 ? 'v-div' : ''}>
						<input
							bind:value={input[rowIndex][colIndex]}
							on:input={(e) => handleInput(e, rowIndex, colIndex)}
							on:keydown={(e) => handleKeydown(e)}
							data-row={rowIndex}
							data-col={colIndex}
							maxlength="1"
						/>
					</td>
				{/each}
			</tr>
		{/each}
	</table>
	<button on:click={submit}> Solve </button>
	<button on:click={reset}> Clear </button>
</div>

<style>
	input {
		box-shadow: none;
		width: 2rem;
		height: 2rem;
		border: none;
		text-align: center;
		font-size: 20px;
	}

	td {
		padding: 0rem;
		border: 1px solid black;
	}

	table {
		border-collapse: collapse;
		border: 3px solid black;
		margin: 1rem;
	}

	.v-div {
		border-right: 3px solid black;
	}

	.h-div {
		border-bottom: 3px solid black;
	}
</style>

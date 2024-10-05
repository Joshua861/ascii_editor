<script>
	import { Tooltip } from 'bits-ui';
	import { flyAndScale } from './fly_and_scale';

	$: ({ disabled = false, tooltip, onClick, className, noClass = false, ...inputProps } = $$props);
</script>

<Tooltip.Root>
	<Tooltip.Trigger>
		<button
			{...inputProps}
			on:click={onClick}
			{disabled}
			class="{noClass
				? ''
				: 'disabled:hover-bg-white my-auto rounded-2xl p-4 hover:bg-neutral-100 disabled:opacity-50'} {className} 
      {disabled ? 'cursor-not-allowed' : 'pointer'}"
		>
			<slot />
		</button>
	</Tooltip.Trigger>
	<Tooltip.Content
		class="z-30"
		transition={flyAndScale}
		transitionConfig={{ y: 8, duration: 150 }}
		sideOffset={8}
	>
		<div class="z-30 bg-neutral-100">
			<Tooltip.Arrow class="rounded-[2px] border-l border-t border-neutral-200" />
		</div>
		<div
			class="border-neutral-200-10 z-30 flex items-center justify-center rounded-xl border bg-neutral-100 p-3 text-sm font-medium shadow outline-none"
		>
			{tooltip}
		</div>
	</Tooltip.Content>
</Tooltip.Root>

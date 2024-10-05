<script lang="ts">
	import { Dialog, Separator } from 'bits-ui';
	import { fade } from 'svelte/transition';
	import { flyAndScale } from './fly_and_scale';
	import { X } from 'lucide-svelte';

	export let title, open: boolean, closeButtonText: string, closeButtonOnClick: () => void;
</script>

<Dialog.Root bind:open>
	<Dialog.Portal>
		<Dialog.Overlay
			transition={fade}
			transitionConfig={{ duration: 150 }}
			class="fixed inset-0 z-50 bg-neutral-500/50"
		/>
		<Dialog.Content
			transition={flyAndScale}
			class="fixed left-[50%] top-[50%] z-50 w-full max-w-[94%] translate-x-[-50%] translate-y-[-50%] rounded-2xl border bg-neutral-100 p-5 shadow-xl outline-none sm:max-w-[490px] md:w-full"
		>
			<Dialog.Title
				class="flex w-full items-center justify-center text-lg font-semibold tracking-tight"
				>{title}</Dialog.Title
			>
			<Separator.Root class="-mx-5 mb-6 mt-5 block h-px bg-neutral-200" />
			<slot />
			<div class="flex w-full justify-end">
				<Dialog.Close
					on:click={closeButtonOnClick}
					class="active:scale-98 inline-flex items-center justify-center rounded-xl bg-neutral-200 px-16 py-3 text-[15px] font-semibold hover:bg-neutral-300 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2"
				>
					{closeButtonText}
				</Dialog.Close>
			</div>
			<Dialog.Close
				class="active:scale-98 absolute right-5 top-5 rounded-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2"
			>
				<div>
					<X />
					<span class="sr-only">Close</span>
				</div>
			</Dialog.Close>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>

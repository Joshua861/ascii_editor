<script>
	import { goto } from '$app/navigation';

	let width = 30,
		height = 18;
</script>

<div class="flex h-[100vh] w-[100vw] items-center sm:text-2xl">
	<div class="mx-auto w-fit">
		<pre>+----------------------------+
|                            |
| # Ascii Editor             |
|                            |
|  Draw ascii art with ease. |
|                            |
|  Width:  <input
				class="w-[96px] text-left outline-none sm:w-[144px]"
				bind:value={width}
				type="text"
			/>        |
|  Height: <input
				class="w-[96px] text-left outline-none sm:w-[144px]"
				bind:value={height}
				type="text"
			/>        |
|                            |
|  <button
				class="hover:underline"
				on:click={() => {
					goto('/draw?width=' + width + '&height=' + height);
				}}>New drawing</button
			>               |
|                            |
|            ---             |
|  <a class="hover:underline" href="/docs/help">Help</a>                      |
|                            |
+----------------------------+</pre>
	</div>
	<pre class="font-2xl">s</pre>
</div>

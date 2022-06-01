<script lang="ts">
	import '../app.css';
</script>

<div class="flex flex-wrap border-b-2 border-pink-500">
	<div class="w-full">
		<nav class="relative flex flex-wrap items-center justify-end px-2 py-3 navbar-expand-lg bg-slate-900">
				<div class="px-3">
					<button class="bg-fuchsia-500 hover:bg-fuchsia-800 px-4 py-3 rounded">Sign In</button>
				</div>
				<div>
					<button class="bg-fuchsia-500 hover:bg-fuchsia-800 px-4 py-3 rounded">Register</button>
				</div>
		</nav>
	</div>
</div>

<main>
	<slot />
</main>

<style global lang="postcss">
	@tailwind base;
	@tailwind components;
	@tailwind utilities;
</style>

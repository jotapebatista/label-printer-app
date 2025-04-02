<template>
	<Html class="min-h-screen scroll-smooth">
		<Body class="overflow-x-hidden bg-slate-800 font-mono">
			<NuxtLayout>
				<NuxtPage />
				<Toaster />
			</NuxtLayout>
		</Body>
	</Html>
</template>

<script setup lang="ts">

</script>

<style lang="scss">
	html {
		-webkit-tap-highlight-color: transparent;
	}
	img {
		-webkit-user-drag: none;
		@apply select-none;
	}

	// Transitions
	.page-enter-active,
	.page-leave-active {
		@apply transition-opacity ease-in-out duration-300;
	}
	.layout-enter-active,
	.layout-leave-active {
		@apply transition-opacity ease-in-out duration-500;
	}
	.page-enter-from,
	.page-leave-to,
	.layout-enter-from,
	.layout-leave-to {
		@apply opacity-0;
	}
</style>

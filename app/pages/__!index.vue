<template>
	<div
		class="flex items-center justify-center p-6 text-light-100"
	>
		<div class="relative max-w-6xl w-full flex gap-6 overflow-hidden">
			<!-- Animated Container -->
			<transition name="slide">
				<div
					v-if="printedStickers.length"
					class="relative w-full flex gap-6 transition-all duration-500"
				>
					<!-- Printer Configuration (Slides Left) -->
					<div
						class="w-1/2 translate-x-[-20%] transform transition-all duration-500"
					>
						<PrinterConfig @submit-sticker="getMacAddress" />
					</div>

					<!-- Printed Stickers (Appears on Right) -->
					<div
						class="w-1/2 overflow-y-auto rounded-lg p-6 shadow-xl"
					>
						<h2 class="mb-4 text-xl font-bold">
							Printed Stickers
						</h2>
						<ul class="text-sm space-y-4">
							<li
								v-for="(sticker, index) in printedStickers"
								:key="index"
								class="rounded-lg bg-gray-800 p-4 text-white shadow-md"
							>
								<strong>MAC:</strong> {{ sticker.macaddress }}
								<br>
								<strong>Serial:</strong>
								{{ sticker.serialnumber }} <br>
								<strong>Reg ID:</strong> {{ sticker.regid }}
							</li>
						</ul>
					</div>
				</div>
			</transition>

			<!-- Centered Printer Config When No Stickers -->
			<div
				v-if="!printedStickers.length"
				class="w-full flex items-center justify-center transition-all duration-500"
			>
				<PrinterConfig @submit-sticker="getMacAddress" />
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
	const printedStickers = ref<any[]>([]);

	const getMacAddress = async (fileName: string) => {
		const response = await fetch(`http://label-server.bstuff:9000/file?file_name=${fileName}`);
		if (response.ok) {
			const result = await response.json();
			if (result.units && Array.isArray(result.units)) {
				printedStickers.value = result.units;
			}
			return;
		}
		console.error("Failed to fetch printed stickers");
	};
</script>

<style scoped>
/* Slide effect */
.slide-enter-active,
.slide-leave-active {
	transition: transform 0.5s ease-in-out, opacity 0.5s ease-in-out;
}
.slide-enter-from {
	transform: translateX(100%);
	opacity: 0;
}
.slide-leave-to {
	transform: translateX(-100%);
	opacity: 0;
}
</style>

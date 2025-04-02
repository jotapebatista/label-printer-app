<template>
	<div class="space-y-2">
		<div v-for="(sticker, index) in qrCodeInputs" :key="index" class="flex justify-between items-center">
			<span>{{ sticker.split(",")[2] }}</span>
			<button class="bg-blue-600 text-white py-2 px-4 rounded" @click="reprint(sticker)">
				Re-print
			</button>
		</div>
	</div>
</template>

  <script setup lang="ts">
	const _props = defineProps({
		qrCodeInputs: {
			type: Array,
			required: true
		}
	});

	const emit = defineEmits(["reprint"]);

	const reprint = (qrCode: string) => {
		emit("reprint", qrCode);
	};
  </script>

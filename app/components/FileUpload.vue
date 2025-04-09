<template>
	<div class="mt-4 p-4 border border-gray-300 rounded-lg bg-white shadow-sm">
		<label class="block text-sm font-medium text-gray-700 mb-2">Upload PDF file</label>
		<input
			type="file"
			accept="application/pdf"
			class="block w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-sm file:font-semibold file:bg-blue-50 file:text-blue-700 hover:file:bg-blue-100"
			@change="handleFileChange"
		>
		<p v-if="fileName" class="mt-2 text-sm text-green-600">
			Selected: {{ fileName }}
		</p>
	</div>
</template>

<script setup lang="ts">
	const emit = defineEmits<{
		(e: "pdf-selected", file: File): void
	}>();

	const fileName = ref<string | null>(null);

	const handleFileChange = (event: Event) => {
		const input = event.target as HTMLInputElement;
		if (input.files && input.files.length > 0) {
			const file = input.files[0];
			fileName.value = file.name;

			if (!file.name.endsWith(".pdf")) {
				alert("The selected file is not a PDF.");
				return;
			}

			emit("pdf-selected", file);
		}
	};
</script>

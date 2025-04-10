<template>
	<div class="mt-4 p-4 border border-gray-300 rounded-lg bg-white shadow-sm">
		<label class="block text-sm font-medium text-gray-700 mb-2">Select or Upload PDF file</label>

		<!-- Dropdown for selecting from server -->
		<select
			v-if="pdfList.length > 0"
			v-model="selectedPdf"
			class="block w-full text-sm text-gray-500 border-gray-300 rounded-md py-2 px-3"
			@change="handleDropdownSelect"
		>
			<option value="">
				-- Select a PDF --
			</option>
			<option v-for="pdf in pdfList" :key="pdf.path" :value="pdf.path">
				{{ pdf.file }}
			</option>
		</select>

		<!-- Or upload a new PDF -->
		<div v-if="pdfList.length === 0 || !isPdfSelected">
			<input
				type="file"
				accept="application/pdf"
				class="block w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-sm file:font-semibold file:bg-blue-50 file:text-blue-700 hover:file:bg-blue-100 mt-4"
				@change="handleFileChange"
			>
		</div>

		<p v-if="fileName" class="mt-2 text-sm text-green-600">
			Selected: {{ fileName }}
		</p>

		<p v-if="uploadStatus" class="mt-2 text-sm" :class="uploadStatusClass">
			{{ uploadStatus }}
		</p>
	</div>
</template>

<script setup lang="ts">
	import { computed, onMounted, ref } from "vue";

	// Emitting the selected PDF file (from dropdown or upload)
	const emit = defineEmits<{
		(e: "pdf-selected", file: File | string): void
		(e: "pdf-inserted", file: File | string): void
	}>();

	const fileName = ref<string | null>(null);
	const uploadStatus = ref<string | null>(null);
	const pdfList = ref<Array<{ file: string, path: string }>>([]);
	const selectedPdf = ref<string>(""); // Track the selected PDF
	const isPdfSelected = ref(false); // Track whether a PDF has been selected or uploaded

	const uploadStatusClass = computed(() => {
		if (!uploadStatus.value)
			return "";
		if (uploadStatus.value.includes("successfully"))
			return "text-green-600";
		if (uploadStatus.value.includes("already exists"))
			return "text-yellow-600";
		return "text-red-600";
	});

	// Fetch list of PDFs from server
	const fetchPdfList = async () => {
		try {
			const response = await fetch("http://label-server.bstuff:6500/api/v2/get_stickers");
			const result = await response.json();
			if (response.ok) {
				// Trigger reactivity by resetting the pdfList reference
				pdfList.value = result.files || [];
				if (pdfList.value.length > 0) {
					fileName.value = pdfList.value[0].file;
					isPdfSelected.value = true;
				}
			} else {
				console.error("Failed to fetch PDFs:", result);
			}
		} catch (error) {
			console.error("Error fetching PDFs:", error);
		}
	};

	// Computed property for updating the dropdown list
	const updatedPdfList = computed(() => {
		return pdfList.value;
	});

	// Handle selecting a PDF from the dropdown
	const handleDropdownSelect = () => {
		const selectedPdfPath = selectedPdf.value;
		if (selectedPdfPath) {
			emit("pdf-selected", selectedPdfPath);
			const selectedFile = updatedPdfList.value.find((pdf) => pdf.path === selectedPdfPath);
			if (selectedFile) {
				fileName.value = selectedFile.file;
				isPdfSelected.value = true;
			}
		} else {
			isPdfSelected.value = false;
		}
	};

	// Handle file upload
	const handleFileChange = (event: Event) => {
		const input = event.target as HTMLInputElement;
		if (input.files && input.files.length > 0) {
			const file = input.files[0];
			fileName.value = file.name;

			emit("pdf-inserted", file);
			fetchPdfList(); // Fetch the updated list after file is uploaded
		}
	};

	// Fetch PDF list on mount
	onMounted(() => {
		fetchPdfList();
	});
</script>

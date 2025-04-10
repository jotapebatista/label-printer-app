<template>
	<div class="flex h-screen">
		<div class="absolute top-4 left-0 right-0 flex justify-center h-18">
			<div class="flex justify-center mb-6">
				<NuxtLink
					to="/v1"
					class="relative inline-flex items-center justify-center px-6 py-3 overflow-hidden font-semibold text-white transition duration-300 ease-out bg-gradient-to-r from-purple-600 to-indigo-600 rounded-xl shadow-xl hover:from-indigo-600 hover:to-purple-600 hover:scale-105"
				>
					<span class="absolute inset-0 bg-gradient-to-br from-indigo-700 to-purple-700 opacity-25 blur-sm" />
					<span class="relative z-10">🚀 Use V1</span>
				</NuxtLink>
			</div>
		</div>

		<div
			v-if="isLoading"
			class="absolute h-screen w-screen z-5 bg-slate-900 bg-opacity-50 flex items-center justify-center"
		>
			<div role="status" class="m-auto">
				<svg
					aria-hidden="true"
					class="w-8 h-8 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600"
					viewBox="0 0 100 101"
					fill="none"
					xmlns="http://www.w3.org/2000/svg"
				>
					<path
						d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
						fill="currentColor"
					/>
					<path
						d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
						fill="currentFill"
					/>
				</svg>
				<span class="sr-only">Loading...</span>
			</div>
		</div>

		<div class="transition-all duration-300 ease-in-out m-auto">
			<!-- Printer Configuration -->
			<div class="bg-slate-600 rounded-md p-4 min-w-xl">
				<div class="w-full flex justify-center items-center flex-col">
					<Logo />
				</div>
				<div class="flex items-center gap-4">
					<div class="w-full">
						<FormSelect
							v-model="selectedPrinter"
							label="Selected Printer"
							:options="availablePrinters"
						/>
					</div>

					<!-- Show switch only if the selected printer is a TSC -->

					<div
						v-if="selectedPrinterBrand === 'tsc'"
						class="flex gap-2 h-full self-end"
					>
						<span class="text-gray-200 text-sm self-end">Double Sticker?</span>
						<label
							for="doubleSticker"
							class="inline-flex items-center space-x-2"
						>
							<input
								id="doubleSticker"
								v-model="isDoubleSticker"
								type="checkbox"
								:class="
									isDoubleSticker
										? 'bg-blue-700'
										: 'bg-gray-300'
								"
								class="h-5 w-5 rounded border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus-visible:ring-2 focus-visible:ring-white/75"
							>
						</label>
					</div>
				</div>

				<div class="flex w-full">
					<!-- <FormSelect
						v-model="selectedFormat"
						label="Label Format"
						:options="labelOptions"
					/> -->
					<FormMultiLabelSelect
						v-model="selectedLabels"
						label="Select Labels and Quantities"
						:options="labelOptions"
						@dataplate-selected="handleDataplate"
					/>
					<!-- <FormInput
						v-model="copies"
						label="Copies"
						type="number"
						:min="1"
					/> -->
				</div>

				<div v-if="isDataplateSelected">
					<FileUpload
						v-if="isDataplateSelected"
						@pdf-inserted="handlePdfUpload"
						@pdf-selected="selectedFile = $event"
					/>
				</div>

				<div class="grid grid-cols-1 gap-4">
					<!-- <FormInput
						v-model="macAddressFile"
						label="Save Mac Address"
						placeholder="Enter file name"
					/> -->
					<FormTextarea
						v-model="qrCodeInput"
						label="QR Code Input"
						placeholder="Scan QR code"
						@input="debouncedPrint"
					/>
				</div>

				<!-- Print Sticker Button -->
				<div class="flex items-center gap-2">
					<button
						class="w-full bg-blue-600 text-white py-3 rounded-sm font-semibold hover:bg-blue-700 transition duration-200 mt-2"
						:disabled="isPrinting"
						:class="{ 'opacity-50 cursor-not-allowed': isPrinting }"
						@click="generatePayload(false)"
					>
						Print Sticker
					</button>
					<button
						v-if="lastInput !== ''"
						class="w-full bg-blue-600 text-white py-3 rounded-sm font-semibold hover:bg-blue-700 transition duration-200 mt-2"
						:disabled="isPrinting && lastInput === ''"
						:class="{ 'opacity-50 cursor-not-allowed': isPrinting }"
						@click="reprintSticker(lastInput)"
					>
						Print Previous
					</button>
				</div>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
	const { addToast } = useToast();

	// const BASE_URL = "http://label-server.bstuff:9000";
	const BASE_URL = "http://label-server.bstuff:6500/api/v2"; //  Change this to the actual server IP

	// ux8 command: python3 main.py -f UX8.pdf -l dataplate nomac nomac nomac -d string -a 192.168.40.226 -p tcp -b tsc-double -n

	const selectedLabels = ref<string[]>([]);
	const selectedPrinter = ref(null);
	const isDataplateSelected = ref(false);
	const qrCodeInput = ref("");
	const lastInput = ref("");
	const isPrinting = ref(false);
	const isLoading = ref(false);
	const uploadStatus = ref<string | null>(null);
	const selectedFile = ref<string | null>(null);

	const labelOptions = ref([]);

	interface Printer {
		label: string
		value: string
		brand: string
	}

	const handleDataplate = (newVal: boolean) => {
		isDataplateSelected.value = newVal;
	};
	const handlePdfUpload = async (file: File) => {
		const formData = new FormData();
		formData.append("pdf_file", file);

		try {
			const response = await fetch(`${BASE_URL}/upload_pdf`, {
				method: "POST",
				body: formData
			});

			const result = await response.json();

			if (response.status === 302) {
				uploadStatus.value = result.message || "File already exists.";
				selectedFile.value = result.file_path;
				showToast(
					"File alaready exists. You can proceed to print.",
					"success",
					3000,
					true
				);
			} else if (response.ok) {
				uploadStatus.value = result.message;
				selectedFile.value = result.file_path;
				showToast(
					"File uploaded successfully.",
					"success",
					3000,
					true
				);
			} else {
				uploadStatus.value = result.error || "Upload failed.";
				selectedFile.value = null;
				showToast(
					"File upload failed.",
					"error",
					3000,
					true
				);
			}
		} catch (error) {
			console.error(error);
			uploadStatus.value = "An error occurred during upload.";
		}
	};
	const availablePrinters = ref<Printer[]>([]);

	const printedStickers = ref<Set<string>>(new Set());
	// const stickersExist = computed(() => printedStickers.value.size > 0);
	// const stickers = computed(() => Array.from(printedStickers.value));
	const selectedPrinterBrand = computed(() => {
		return (
			availablePrinters.value.find((p) => p.value === selectedPrinter.value)
				?.brand || null
		);
	});
	// Show toasts
	const showToast = (
		msg: string,
		type: "success" | "error" | "warning" | "info" | undefined,
		duration: number,
		closable: boolean
	) => {
		addToast(msg, type, duration, closable);
	};

	const fetchData = async () => {
		try {
			isLoading.value = true;
			const response = await fetch(
				`${BASE_URL}/file?file_name=printers.json`
			);
			const printers = await response.json();
			availablePrinters.value = printers.map((printer: any) => ({
				label: printer.label,
				value: printer.value,
				brand: printer.brand
			}));

			const labelResponse = await fetch(
				`${BASE_URL}/file?file_name=labels.json`
			);
			const labels = await labelResponse.json();
			labelOptions.value = labels.map((label: any) => ({
				label: label.label,
				value: label.value
			}));
			isLoading.value = false;
		} catch (error) {
			showToast(`Error fetching data: ${error}`, "error", 3000, true);
			isLoading.value = false;
		}
	};

	onMounted(() => {
		fetchData();
	});

	const isDoubleSticker = ref<boolean>(false);

	let debounceTimer: any = null;

	const detectInputType = () => {
		try {
			JSON.parse(qrCodeInput.value);
			return "json";
		} catch {
			return "string";
		}
	};

	// Generate Payload to Print Sticker
	const generatePayload = async (printLast = false) => {
		isPrinting.value = true;

		const brand = availablePrinters.value.find(
			(printer) => printer.value === selectedPrinter.value
		)?.brand;

		const pBrand
			= brand === "tsc" && isDoubleSticker.value
				? `${brand}-double`
				: brand || "";

		const payload: Record<string, any> = {
			brand: pBrand,
			address: selectedPrinter.value,
			protocol: "tcp",
			label_format: selectedLabels.value,
			input_data: printLast ? lastInput.value : qrCodeInput.value,
			data_format: detectInputType(),
			test_mode: false,
			no_check_duplicate: true
		};

		if (selectedFile.value) {
			payload.image_file = selectedFile.value;
		}

		const jsonPayload = JSON.stringify(payload);

		try {
			const response = await fetch(`${BASE_URL}/print_labels`, {
				method: "POST",
				headers: { "Content-Type": "application/json" },
				body: jsonPayload
			});

			if (response.ok) {
				let stickersArray = Array.from(printedStickers.value);
				stickersArray = stickersArray.filter(
					(item) => item !== qrCodeInput.value
				);

				stickersArray.unshift(qrCodeInput.value);

				printedStickers.value = new Set(stickersArray.slice(0, 50));

				qrCodeInput.value = "";
				lastInput.value = qrCodeInput.value;
				showToast("Sticker printed successfully", "success", 3000, true);
				isPrinting.value = false;
			} else {
				showToast(
					`Error printing sticker: ${response.statusText}`,
					"error",
					3000,
					true
				);
				console.error("Error printing sticker:", response.statusText);
			}
			isPrinting.value = false;
		} catch (error) {
			// notify("Error printing sticker", "error");
			showToast(`Error printing sticker: ${error}`, "error", 3000, true);
			console.error("Error printing sticker:", error);
		}
	};

	// Debounced QR Code Input Handling
	const debouncedPrint = () => {
		clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => {
			generatePayload(false);
		}, 500);
	};

	const reprintSticker = (qrCode: string) => {
		generatePayload(true);
	};
</script>

<style scoped>
.stickers-transition-enter-active,
.stickers-transition-leave-active {
	transition: transform 0.3s ease-in-out, opacity 0.3s ease-in-out;
}
.stickers-transition-enter,
.stickers-transition-leave-to {
	transform: translateX(100%);
	opacity: 0;
}
</style>

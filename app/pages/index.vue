<template>
	<div
		class="min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900"
	>
		<!-- Loading Overlay -->
		<div
			v-if="isLoading"
			class="fixed inset-0 z-50 bg-black/50 backdrop-blur-sm flex items-center justify-center"
		>
			<div
				class="bg-white/10 backdrop-blur-md rounded-2xl p-8 border border-white/20"
			>
				<div class="flex flex-col items-center gap-4">
					<div class="relative">
						<div
							class="w-16 h-16 border-4 border-blue-500/30 border-t-blue-500 rounded-full animate-spin"
						/>
						<div
							class="absolute inset-0 w-16 h-16 border-4 border-transparent border-t-blue-400 rounded-full animate-spin"
							style="animation-delay: -0.5s"
						/>
					</div>
					<p class="text-white font-medium">
						Connecting to label server...
					</p>
				</div>
			</div>
		</div>

		<!-- Main Layout -->
		<div class="container mx-auto px-4 py-8">
			<!-- Header -->
			<div class="text-center mb-8">
				<div
					class="inline-flex items-center gap-4 bg-white/5 backdrop-blur-sm rounded-2xl px-8 py-4 border border-white/10"
				>
					<Logo class="mb-8" />
					<!-- <div class="h-8 w-px bg-white/20" />
					<div class="text-left">
						<h1 class="text-2xl font-bold text-white">
							Label Printer
						</h1>
						<p class="text-sm text-white/60">
							Industrial Label Management System
						</p>
					</div> -->
				</div>
			</div>

			<!-- Main Content -->
			<div class="grid grid-cols-1 xl:grid-cols-3 gap-8">
				<!-- Left Panel - Configuration -->
				<div class="xl:col-span-2 space-y-6">
					<!-- Printer Configuration Card -->
					<div
						class="bg-white/5 backdrop-blur-sm rounded-2xl border border-white/10 relative z-20"
					>
						<div
							class="bg-gradient-to-r from-blue-500/20 to-purple-500/20 px-6 py-4 border-b border-white/10"
						>
							<h2
								class="text-lg font-semibold text-white flex items-center gap-2"
							>
								<svg
									class="w-5 h-5"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M17.657 18.657A8 8 0 016.343 7.343S7 9 9 10c0-2 .5-5 2.986-7C14 5 16.09 5.777 17.656 7.343A7.975 7.975 0 0120 13a7.975 7.975 0 01-2.343 5.657z"
									/>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M9.879 16.121A3 3 0 1012.015 11L11 14H9c0 .768.293 1.536.879 2.121z"
									/>
								</svg>
								Printer Configuration
							</h2>
						</div>
						<div class="p-6 space-y-4">
							<div
								class="grid grid-cols-1 lg:grid-cols-2 gap-4 h-full"
							>
								<FormSelect
									v-model="selectedPrinter"
									label="Select Printer"
									:options="availablePrinters"
								/>
								<FormSelect
									v-if="selectedPrinterBrand === 'tsc'"
									v-model="selectedLabelSize"
									label="Label Size"
									:options="labelSizes"
								/>
							</div>
						</div>
					</div>

					<!-- Label Configuration Card -->
					<div
						class="bg-white/5 backdrop-blur-sm rounded-2xl border border-white/10 relative z-10"
					>
						<div
							class="bg-gradient-to-r from-green-500/20 to-emerald-500/20 px-6 py-4 border-b border-white/10"
						>
							<h2
								class="text-lg font-semibold text-white flex items-center gap-2"
							>
								<svg
									class="w-5 h-5"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z"
									/>
								</svg>
								Label Configuration
							</h2>
						</div>
						<div class="p-6 space-y-4">
							<FormMultiLabelSelect
								v-model="selectedLabels"
								label="Select Labels & Quantities"
								:options="labelOptions"
								@dataplate-selected="handleDataplate"
							/>

							<!-- File Upload Section -->
							<div
								v-if="isDataplateSelected"
								class="bg-white/5 rounded-xl p-4 border border-white/10"
							>
								<FileUpload
									@pdf-inserted="handlePdfUpload"
									@pdf-selected="handlePdfSelected"
								/>
							</div>
						</div>
					</div>
				</div>

				<!-- Right Panel - Input & Actions -->
				<div class="space-y-6">
					<!-- QR Code Input Card -->
					<div
						class="bg-white/5 backdrop-blur-sm rounded-2xl border border-white/10"
					>
						<div
							class="bg-gradient-to-r from-orange-500/20 to-red-500/20 px-6 py-4 border-b border-white/10"
						>
							<h2
								class="text-lg font-semibold text-white flex items-center gap-2"
							>
								<svg
									class="w-5 h-5"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M12 4v1m6 11h2m-6 0h-2v4m0-11v3m0 0h.01M12 12h4.01M16 20h4M4 12h4m12 0h.01M5 8h2a1 1 0 001-1V6a1 1 0 00-1-1H5a1 1 0 00-1 1v1a1 1 0 001 1zm12 0h2a1 1 0 001-1V6a1 1 0 00-1-1h-2a1 1 0 00-1 1v1a1 1 0 001 1zM5 20h2a1 1 0 001-1v-1a1 1 0 00-1-1H5a1 1 0 00-1 1v1a1 1 0 001 1z"
									/>
								</svg>
								QR Code Scanner
							</h2>
						</div>
						<div class="p-6">
							<FormTextarea
								v-model="qrCodeInput"
								label="Scan QR Code or Enter Data"
								placeholder="Place QR code under scanner or type manually..."
								@input="debouncedPrint"
							/>
						</div>
					</div>

					<!-- Action Buttons Card -->
					<div
						class="bg-white/5 backdrop-blur-sm rounded-2xl border border-white/10"
					>
						<div
							class="bg-gradient-to-r from-indigo-500/20 to-blue-500/20 px-6 py-4 border-b border-white/10"
						>
							<h2
								class="text-lg font-semibold text-white flex items-center gap-2"
							>
								<svg
									class="w-5 h-5"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M17 17h2a2 2 0 002-2v-4a2 2 0 00-2-2H5a2 2 0 00-2 2v4a2 2 0 002 2h2m2 4h6a2 2 0 002-2v-4a2 2 0 00-2-2H9a2 2 0 00-2 2v4a2 2 0 002 2zm8-12V5a2 2 0 00-2-2H9a2 2 0 00-2 2v4h10z"
									/>
								</svg>
								Print Actions
							</h2>
						</div>
						<div class="p-6 space-y-3">
							<button
								class="w-full bg-gradient-to-r from-blue-600 to-blue-700 hover:from-blue-700 hover:to-blue-800 text-white py-4 px-6 rounded-xl font-semibold transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-3 shadow-lg hover:shadow-xl transform hover:scale-[1.02]"
								:disabled="isPrinting || qrCodeInput === ''"
								@click="generatePayload(false)"
							>
								<svg
									v-if="isPrinting"
									class="w-6 h-6 animate-spin"
									fill="none"
									viewBox="0 0 24 24"
								>
									<circle
										class="opacity-25"
										cx="12"
										cy="12"
										r="10"
										stroke="currentColor"
										stroke-width="4"
									/>
									<path
										class="opacity-75"
										fill="currentColor"
										d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
									/>
								</svg>
								<svg
									v-else
									class="w-6 h-6"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M17 17h2a2 2 0 002-2v-4a2 2 0 00-2-2H5a2 2 0 00-2 2v4a2 2 0 002 2h2m2 4h6a2 2 0 002-2v-4a2 2 0 00-2-2H9a2 2 0 00-2 2v4a2 2 0 002 2zm8-12V5a2 2 0 00-2-2H9a2 2 0 00-2 2v4h10z"
									/>
								</svg>
								{{ isPrinting ? "Printing..." : "Print Label" }}
							</button>

							<button
								v-if="lastInput !== ''"
								class="w-full bg-gradient-to-r from-slate-600 to-slate-700 hover:from-slate-700 hover:to-slate-800 text-white py-4 px-6 rounded-xl font-semibold transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-3 shadow-lg hover:shadow-xl transform hover:scale-[1.02]"
								:disabled="isPrinting"
								@click="reprintSticker(lastInput)"
							>
								<svg
									class="w-6 h-6"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
									/>
								</svg>
								Print Previous
							</button>
						</div>
					</div>

					<!-- Status Card -->
					<div
						class="bg-white/5 backdrop-blur-sm rounded-2xl border border-white/10"
					>
						<div
							class="bg-gradient-to-r from-emerald-500/20 to-teal-500/20 px-6 py-4 border-b border-white/10"
						>
							<h2
								class="text-lg font-semibold text-white flex items-center gap-2"
							>
								<svg
									class="w-5 h-5"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
									/>
								</svg>
								Status
							</h2>
						</div>
						<div class="p-6 space-y-3">
							<div class="flex items-center justify-between">
								<span class="text-white/60">Printer Status</span>
								<span class="text-emerald-400 font-medium">Connected</span>
							</div>
							<div class="flex items-center justify-between">
								<span class="text-white/60">Labels Selected</span>
								<span class="text-blue-400 font-medium">{{
									selectedLabels.length
								}}</span>
							</div>
							<div class="flex items-center justify-between">
								<span class="text-white/60">Last Print</span>
								<span class="text-white/80 font-medium">{{
									lastInput || "None"
								}}</span>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
	const { addToast } = useToast();

	// const BASE_URL = "http://label-server.bstuff:9000";
	const BASE_URL = "http://192.168.40.182:6500/api/v2"; //  Change this to the actual server IP

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
	const labelSizes = ref([]);
	const selectedLabelSize = ref(null);

	const labelOptions = ref([]);

	interface Printer {
		label: string
		value: string
		brand: string
	}

	// Show toasts
	const showToast = (
		msg: string,
		type: "success" | "error" | "warning" | "info" | undefined,
		duration: number,
		closable: boolean
	) => {
		addToast(msg, type, duration, closable);
	};

	const handleDataplate = (newVal: boolean) => {
		isDataplateSelected.value = newVal;
	};

	const handlePdfUpload = async (file: File | string) => {
		if (typeof file === "string") {
			selectedFile.value = file;
			return;
		}

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
					"File already exists. You can proceed to print.",
					"success",
					3000,
					true
				);
			} else if (response.ok) {
				uploadStatus.value = result.message;
				selectedFile.value = result.file_path;
				showToast("File uploaded successfully.", "success", 3000, true);
			} else {
				uploadStatus.value = result.error || "Upload failed.";
				selectedFile.value = null;
				showToast("File upload failed.", "error", 3000, true);
			}
		} catch (error) {
			console.error(error);
			uploadStatus.value = "An error occurred during upload.";
		}
	};

	const handlePdfSelected = (file: File | string) => {
		if (typeof file === "string") {
			selectedFile.value = file;
		} else {
			selectedFile.value = file.name;
		}
	};

	const availablePrinters = ref<Printer[]>([]);

	const printedStickers = ref<Set<string>>(new Set());

	const selectedPrinterBrand = computed(() => {
		return (
			availablePrinters.value.find((p) => p.value === selectedPrinter.value)
				?.brand || null
		);
	});

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

			const sizes = await fetch(
				`${BASE_URL}/file?file_name=label_sizes.json`
			);
			labelSizes.value = await sizes.json();

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
		if (qrCodeInput.value === "") {
			return;
		}

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
			paper_size: selectedLabelSize.value,
			check_duplicates: false
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

	const reprintSticker = (_qrCode: string) => {
		generatePayload(true);
	};
</script>

<style scoped>
/* Custom scrollbar */
::-webkit-scrollbar {
	width: 8px;
}

::-webkit-scrollbar-track {
	background: rgba(255, 255, 255, 0.1);
	border-radius: 4px;
}

::-webkit-scrollbar-thumb {
	background: rgba(255, 255, 255, 0.3);
	border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
	background: rgba(255, 255, 255, 0.5);
}

/* Smooth transitions */
* {
	transition: all 0.2s ease-in-out;
}

/* Glass morphism effect */
.glass {
	backdrop-filter: blur(16px);
	-webkit-backdrop-filter: blur(16px);
}
</style>

<template>
	<div class="flex h-screen">
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
						<Switch
							v-model="isDoubleSticker"
							:class="
								isDoubleSticker ? 'bg-blue-700' : 'bg-gray-300'
							"
							class="relative inline-flex h-[1.5rem] w-[3rem] shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus-visible:ring-2 focus-visible:ring-white/75 self-center"
						>
							<span class="sr-only">Use setting</span>
							<span
								aria-hidden="true"
								:class="
									isDoubleSticker
										? 'translate-x-5'
										: 'translate-x-0'
								"
								class="pointer-events-none inline-block h-[1.3rem] w-[1.3rem] transform rounded-full bg-white shadow-lg ring-0 transition duration-200 ease-in-out"
							/>
						</Switch>
					</div>
				</div>
				<div class="grid grid-cols-2 gap-4">
					<FormSelect
						v-model="selectedFormat"
						label="Label Format"
						:options="labelOptions"
					/>
					<FormInput
						v-model="copies"
						label="Copies"
						type="number"
						:min="1"
					/>
				</div>
				<div class="grid grid-cols-1 gap-4">
					<FormInput
						v-model="macAddressFile"
						label="Save Mac Address"
						placeholder="Enter file name"
					/>
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
						@click="generatePayload"
					>
						Print Sticker
					</button>
				</div>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
	const { addToast } = useToast();

	const selectedFormat = ref(null);
	const selectedPrinter = ref(null);
	const copies = ref(1);
	const macAddressFile = ref("");
	const qrCodeInput = ref("");
	const lastInput = ref("");
	const isPrinting = ref(false);
	const isDoubleSticker = ref(false);
	let debounceTimer: any = null;
	const BASE_URL = "http://label-server.bstuff:9000";

	const labelOptions = ref([{ label: "Choose the label format", value: null }]);

	const availablePrinters = ref([
		{ label: "Choose an available printer", value: null, brand: null }
	]);
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
		} catch (error) {
			showToast("Error fetching data", "error", 3000, true);
		}
	};

	onMounted(() => {
		fetchData();
	});

	const detectInputType = () => {
		try {
			JSON.parse(qrCodeInput.value);
			return "json";
		} catch {
			return "string";
		}
	};

	const generatePayload = async () => {
		isPrinting.value = true;
		const brand = availablePrinters.value.find(
			(printer) => printer.value === selectedPrinter.value
		)?.brand;

		const pBrand
			= brand === "tsc" && isDoubleSticker.value
				? `${brand}-double`
				: brand || "";

		const jsonPayload = JSON.stringify({
			brand: pBrand,
			address: selectedPrinter.value,
			protocol: "tcp",
			label_format: selectedFormat.value,
			input_data: qrCodeInput.value,
			output_file: macAddressFile.value,
			copies: copies.value,
			data_format: detectInputType(),
			test_mode: false
		});

		try {
			const response = await fetch(`${BASE_URL}/print`, {
				method: "POST",
				headers: { "Content-Type": "application/json" },
				body: jsonPayload
			});

			if (response.ok) {
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

	const debouncedPrint = () => {
		clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => {
			generatePayload();
		}, 500);
	};
</script>

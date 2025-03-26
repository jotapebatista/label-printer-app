<template>
	<div
		class="max-h-screen flex items-center justify-center text-dark-900 dark:text-light-100"
	>
		<div class="mt-12 max-w-5xl w-full flex gap-6">
			<!-- Printer Configuration (Left) -->
			<UCard
				class="p-6"
				:class="[printedStickers.length ? 'w-1/2' : 'w-full']"
				:ui="{ body: { padding: 'p-6' } }"
			>
				<h1 class="mb-6 text-center text-2xl font-bold">
					Printer Configuration
				</h1>

				<UFormGroup label="Printer Brand" class="mb-4">
					<USelect v-model="brand" :options="printerOptions" />
				</UFormGroup>

				<UFormGroup label="Printer IP" class="mb-4">
					<UInput v-model="ipAddr" placeholder="Enter printer IP" />
				</UFormGroup>

				<UFormGroup label="Label Format" class="mb-4">
					<USelect v-model="labelFormat" :options="labelOptions" />
				</UFormGroup>

				<UFormGroup label="Copies" class="mb-4">
					<UInput v-model="copies" type="number" />
				</UFormGroup>

				<UFormGroup label="Save Mac Address" class="mb-4">
					<UInput v-model="macsFile" placeholder="Enter filename" />
				</UFormGroup>

				<UFormGroup label="Data Format" class="mb-4">
					<USelect
						v-model="dataFormat"
						:options="dataFormatOptions"
					/>
				</UFormGroup>

				<UFormGroup label="QR Code Input" class="mb-4">
					<UTextarea
						v-model="qrInput"
						placeholder="Scan QR code"
						@input="debouncedPrint"
					/>
				</UFormGroup>

				<UFormGroup label="Test Mode" class="mb-4">
					<UToggle v-model="testMode" />
				</UFormGroup>

				<UButton
					block
					color="primary"
					variant="solid"
					:disabled="!isFormValid"
					@click="generatePayload"
				>
					Print Sticker
				</UButton>
			</UCard>

			<!-- Printed Stickers (Right, Scrollable) -->
			<UCard
				v-if="printedStickers.length"
				class="h-full max-h-[calc(100vh-15rem)] w-1/2 overflow-y-auto"
			>
				<h2 class="mb-2 text-lg font-bold">
					Printed Stickers
				</h2>
				<ul class="text-sm">
					<li
						v-for="(sticker, index) in printedStickers"
						:key="index"
						class="mb-1 border-b pb-2"
					>
						<strong>MAC:</strong> {{ sticker.macaddress }}<br>
						<strong>Serial:</strong> {{ sticker.serialnumber
						}}<br>
						<strong>Reg ID:</strong> {{ sticker.regid }}
					</li>
				</ul>
			</UCard>
		</div>
	</div>
</template>

<script setup lang="ts">
	const brand = ref("");
	const ipAddr = ref("");
	const labelFormat = ref("");
	const copies = ref(1);
	const dataFormat = ref("string");
	const qrInput = ref("");
	const macsFile = ref("");
	const apiResponse = ref<string | null>(null);
	const apiSuccess = ref<boolean>(false);
	const printedStickers = ref<any[]>([]);
	const lastPrintedInput = ref<string | null>(null);
	const testMode = ref<boolean>(false);
	const chatEl = ref<HTMLElement>();
	let debounceTimer = null;

	const printerOptions = [
		{ label: "Choose a printer type", value: "" },
		{ label: "Brother", value: "brother" },
		{ label: "TSC", value: "tsc" },
		{ label: "TSC - Double Sticker", value: "tsc-double" }
	];
	const labelOptions = [
		{ label: "Choose the label format", value: "" },
		{ label: "AE", value: "ae" },
		{ label: "Barix", value: "barix" },
		{ label: "NoMac", value: "nomac" },
		{ label: "EdgePlayer", value: "edgeplayer" },
		{ label: "Legacy", value: "barix-legacy" }
	];

	const dataFormatOptions = [
		{ label: "Choose QR Code data format", value: "" },
		{ label: "String", value: "string" },
		{ label: "Json", value: "json" }
	];

	const qrPattern
		= /^\d+,[0-9a-f:]+,[0-9a-z]+,[0-9a-z]+,[0-9a-z]+,\d{2}\/\d{2}\/\d{4},\w+,\w+,\d+,\d+,[0-9a-z ]+,[\w-]+,[0-9.]+,[0-9.]+,[0-9.]+,.*$/i;

	const errors = ref({
		brand: false,
		ipAddr: false,
		labelFormat: false,
		qrInput: false
	});

	const debouncedPrint = () => {
		clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => {
			generatePayload();
		}, 500);
	};

	const isFormValid = computed(() => {
		return brand.value && ipAddr.value && labelFormat.value && qrInput.value;
	});

	watch(printedStickers, (newVal) => {
		if (newVal.length) {
			setTimeout(() => {
				if (chatEl.value) {
					chatEl.value.scrollTo({
						top: chatEl.value.scrollHeight,
						behavior: "smooth"
					});
				}
			}, 500);
		}
	});

	const validateForm = () => {
		errors.value.brand = !brand.value;
		errors.value.ipAddr = !ipAddr.value;
		errors.value.labelFormat = !labelFormat.value;
		errors.value.qrInput = !qrInput.value;
		return isFormValid.value;
	};

	const fetchPrintedStickers = async () => {
		try {
			const response = await fetch(
				`http://label-server.bstuff:9000/file?file_name=${macsFile.value}`
			);
			const result = await response.json();

			if (result.units && Array.isArray(result.units)) {
				printedStickers.value = result.units;
			} else {
				printedStickers.value = [];
				console.warn("Unexpected response format:", result);
			}

			console.log("Printed stickers:", printedStickers.value);
		} catch (error) {
			console.error("Error fetching printed stickers:", error);
		}
	};

	const generatePayload = async () => {
		if (!validateForm())
			return;

		const jsonPayload = JSON.stringify({
			brand: brand.value,
			address: ipAddr.value,
			protocol: "tcp",
			label_format: labelFormat.value,
			input_data: qrInput.value,
			output_file: macsFile.value,
			copies: copies.value,
			data_format: dataFormat.value,
			test_mode: false
		});

		try {
			const response = await fetch("http://label-server.bstuff:9000/print", {
				method: "POST",
				headers: { "Content-Type": "application/json" },
				body: jsonPayload
			});
			lastPrintedInput.value = qrInput.value;

			const result = await response.json();
			apiSuccess.value = response.ok;
			apiResponse.value = response.ok
				? `Success: ${JSON.stringify(result)}`
				: `Error: ${JSON.stringify(result)}`;

			if (response.ok || labelFormat.value === "nomac") {
				qrInput.value = "";
			}

			if (macsFile.value) {
				await fetchPrintedStickers();
			}
		} catch (error) {
			apiSuccess.value = false;
			apiResponse.value = `Error: ${error.message}`;
		}
	};

	const onQrInput = () => {
		console.log(qrPattern.test(qrInput.value));
		if (qrPattern.test(qrInput.value)) {
			generatePayload();
		}
	};
</script>

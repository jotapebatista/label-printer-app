<template>
	<div
		class="max-h-screen flex items-center justify-center text-dark-900 dark:bg-dark-800 dark:text-light-100"
	>
		<div class="mt-12 max-w-5xl w-full flex gap-6">
			<!-- Printer Configuration (Left) -->
			<div
				class="w-1/2 rounded-lg bg-slate-800 p-6 shadow-lg dark:bg-dark-800"
			>
				<h1 class="mb-6 text-center text-2xl text-white font-bold">
					Printer Configuration
				</h1>

				<!-- Printer Brand -->
				<div class="mb-4">
					<label for="brand" class="mb-2 block text-sm text-white font-medium">Printer Brand</label>
					<select id="brand" v-model="brand" class="form-input">
						<option value="">
							Escolha uma opção
						</option>
						<option value="brother">
							Brother
						</option>
						<option value="tsc">
							TSC
						</option>
						<option value="tsc-double">
							TSC - Double Sticker
						</option>
					</select>
				</div>

				<!-- Printer IP -->
				<div class="mb-4">
					<label for="ipAddr" class="mb-2 block text-sm text-white font-medium">Printer IP</label>
					<input id="ipAddr" v-model.trim="ipAddr" type="text" class="form-input">
				</div>

				<!-- Label Format -->
				<div class="mb-4">
					<label for="labelFormat" class="mb-2 block text-sm text-white font-medium">Label Format</label>
					<select id="labelFormat" v-model="labelFormat" class="form-input">
						<option value="">
							Escolha o formato de etiqueta
						</option>
						<option value="ae">
							AE
						</option>
						<option value="barix">
							Barix
						</option>
						<option value="nomac">
							NoMac
						</option>
						<option value="edgeplayer">
							EdgePlayer
						</option>
					</select>
				</div>

				<!-- Save Mac Address -->
				<div class="mb-4">
					<label for="macsFile" class="mb-2 block text-sm text-white font-medium">Save Mac Address</label>
					<input id="macsFile" v-model="macsFile" type="text" class="form-input">
				</div>

				<!-- QR Code Input -->
				<div class="mb-4">
					<label for="qrInput" class="mb-2 block text-sm text-white font-medium">QR Code Input</label>
					<input id="qrInput" v-model="qrInput" type="text" class="form-input">
				</div>

				<!-- Print Button -->
				<button
					class="w-full rounded-lg bg-blue-500 px-4 py-2 text-white font-medium transition disabled:bg-gray-500 hover:bg-blue-600"
					:disabled="!isFormValid"
					@click="generatePayload"
				>
					Print Sticker
				</button>
			</div>

			<!-- Printed Stickers (Right, Scrollable) -->
			<div
				v-if="printedStickers.length"
				class="max-h-screen w-1/2 overflow-y-auto rounded-lg bg-light-100 p-4 shadow-lg dark:bg-dark-700"
			>
				<h2 class="mb-2 text-lg font-bold">
					Printed Stickers
				</h2>
				<ul class="text-sm">
					<li
						v-for="(sticker, index) in printedStickers"
						:key="index"
						class="mb-1 border-b border-gray-300 pb-2"
					>
						<strong>MAC:</strong> {{ sticker.macaddress }} <br>
						<strong>Serial:</strong> {{ sticker.serialnumber }} <br>
						<strong>Reg ID:</strong> {{ sticker.regid }}
					</li>
				</ul>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
	import { computed, onMounted, ref } from "vue";

	const brand = ref("");
	const ipAddr = ref("");
	const labelFormat = ref("");
	const copies = ref(1);
	const qrInput = ref("");
	const macsFile = ref("");
	const apiResponse = ref<string | null>(null);
	const apiSuccess = ref<boolean>(false);
	const printedStickers = ref<any[]>([]);
	const lastPrintedInput = ref<string | null>(null);

	const errors = ref({
		brand: false,
		ipAddr: false,
		labelFormat: false,
		qrInput: false
	});

	const isFormValid = computed(() => {
		return brand.value && ipAddr.value && labelFormat.value && qrInput.value;
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
			data_format: "string"
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

			if (response.ok) {
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

	onMounted(async () => {
		fetchPrintedStickers();
	});
</script>

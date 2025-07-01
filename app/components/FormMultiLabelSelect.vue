<template>
	<div class="w-full">
		<label class="block text-sm font-medium text-gray-200 mb-1">{{ label }}</label>
		<Listbox v-model="selectedLabels" multiple>
			<div class="relative">
				<ListboxButton
					class="relative w-full cursor-pointer rounded-lg border border-gray-300 bg-white py-3 pl-4 pr-10 text-left shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 text-gray-900"
				>
					<div class="flex flex-wrap gap-1">
						<span v-if="selectedLabels.length === 0" class="text-gray-500">
							Select labels...
						</span>
						<span
							v-for="label in selectedLabels"
							:key="label"
							class="inline-flex items-center gap-1 bg-blue-100 text-blue-700 px-2 py-1 rounded-full text-sm"
						>
							{{ getLabelName(label) }}
							<button
								type="button"
								class="text-blue-500 hover:text-blue-700 ml-1"
								@click.stop="removeLabel(label)"
							>
								&times;
							</button>
						</span>
					</div>
					<span class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-3">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="w-5 h-5 text-gray-400"
							fill="none"
							viewBox="0 0 24 24"
							stroke="currentColor"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M19 9l-7 7-7-7"
							/>
						</svg>
					</span>
				</ListboxButton>

				<transition
					leave-active-class="transition duration-100 ease-in"
					leave-from-class="opacity-100"
					leave-to-class="opacity-0"
				>
					<ListboxOptions
						class="absolute z-10 mt-1 w-full max-h-42 overflow-auto rounded-lg bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none border border-gray-200"
					>
						<div
							v-for="option in options"
							:key="option.value"
							class="px-4 py-2 border-b last:border-none hover:bg-gray-50"
						>
							<label class="flex justify-between items-center text-sm text-gray-800 space-x-2 cursor-pointer">
								<div class="flex items-center space-x-2">
									<input
										:checked="selectedLabels.includes(option.value)"
										type="checkbox"
										class="form-checkbox text-blue-600 rounded focus:ring-blue-500"
										@change="toggleLabel(option.value)"
									>
									<span>{{ option.label }}</span>
								</div>
								<input
									v-if="selectedLabels.includes(option.value)"
									v-model.number="labelQuantities[option.value]"
									type="number"
									min="1"
									class="w-16 border-gray-300 rounded-md text-sm px-2 py-1 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-blue-400 text-white"
									@change="updateQuantities"
								>
							</label>
						</div>
					</ListboxOptions>
				</transition>
			</div>
		</Listbox>
	</div>
</template>

<script setup lang="ts">
	import { Listbox, ListboxButton, ListboxOptions } from "@headlessui/vue";
	import { computed, ref, watch } from "vue";

	const props = defineProps<{
		modelValue: string[]
		label: string
		options: { label: string, value: string }[]
	}>();

	const emit = defineEmits(["update:modelValue", "dataplateSelected"]);

	// Internal state
	const selectedLabels = ref<string[]>(props.modelValue || []);
	const labelQuantities = ref<Record<string, number>>({});

	// Initialize quantities
	watch(
		() => props.options,
		(newOptions) => {
			newOptions.forEach((opt) => {
				if (!labelQuantities.value[opt.value]) {
					labelQuantities.value[opt.value] = 1;
				}
			});
		},
		{ immediate: true }
	);

	// Watch for dataplate selection
	watch(
		selectedLabels,
		(newLabels) => {
			const hasDataplate = newLabels.includes("dataplate");
			emit("dataplateSelected", hasDataplate);
		},
		{ immediate: true }
	);

	// Emit updated modelValue with quantities
	const emitUpdatedValue = () => {
		const result: string[] = [];
		selectedLabels.value.forEach((label) => {
			const quantity = labelQuantities.value[label] || 1;
			for (let i = 0; i < quantity; i++) {
				result.push(label);
			}
		});
		emit("update:modelValue", result);
	};

	// Watch for external modelValue changes
	watch(
		() => props.modelValue,
		(newValue) => {
			if (newValue && newValue.length > 0) {
				// Count occurrences of each label
				const labelCounts: Record<string, number> = {};
				newValue.forEach((label) => {
					labelCounts[label] = (labelCounts[label] || 0) + 1;
				});

				// Update selected labels and quantities
				selectedLabels.value = Object.keys(labelCounts);
				labelQuantities.value = { ...labelCounts };
			} else {
				selectedLabels.value = [];
			}
		},
		{ immediate: true }
	);

	// Methods
	const toggleLabel = (label: string) => {
		const index = selectedLabels.value.indexOf(label);
		if (index > -1) {
			selectedLabels.value.splice(index, 1);
		} else {
			selectedLabels.value.push(label);
		}
		emitUpdatedValue();
	};

	const removeLabel = (label: string) => {
		const index = selectedLabels.value.indexOf(label);
		if (index > -1) {
			selectedLabels.value.splice(index, 1);
			emitUpdatedValue();
		}
	};

	const updateQuantities = () => {
		emitUpdatedValue();
	};

	const getLabelName = (value: string) => {
		return props.options.find((opt) => opt.value === value)?.label || value;
	};
</script>

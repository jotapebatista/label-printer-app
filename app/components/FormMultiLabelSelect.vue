<template>
	<div class="w-full">
		<label class="block text-sm font-medium text-gray-200 mb-2">{{
			label
		}}</label>
		<Listbox v-slot="{ open }" as="div" class="max-w-[99.9%]">
			<div class="relative">
				<!-- Fixed height for the ListboxButton with w-full to prevent horizontal expansion -->
				<ListboxButton
					class="relative cursor-pointer rounded-md border border-gray-300 bg-white py-2 pl-3 pr-10 text-left shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 h-12 w-full overflow-x-auto"
				>
					<!-- Horizontal scroll container for selected labels -->
					<div class="flex flex-col w-full">
						<div
							class="text-gray-800 h-8 overflow-x-auto whitespace-nowrap"
						>
							<!-- Wrap the items in a horizontal flex row -->
							<div class="flex space-x-2">
								<div
									v-for="(item, index) in selectedItems"
									:key="`${item.label}-${index}`"
									class="flex items-center space-x-1 bg-blue-100 text-blue-700 px-2 py-1 rounded-full text-sm cursor-pointer"
									@click="removeSelected(item)"
								>
									<span>{{ item.label }} ×{{
										item.count
									}}</span>
									<button
										type="button"
										class="text-blue-500 hover:text-blue-700"
										@click.stop="removeSelected(item)"
									>
										&times;
									</button>
								</div>
							</div>
						</div>
					</div>

					<!-- Dropdown indicator -->
					<span
						class="absolute inset-y-0 right-0 flex items-center pr-2 pointer-events-none"
					>
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

				<!-- Dropdown options -->
				<Transition
					enter="transition duration-100 ease-out"
					enter-from="transform scale-95 opacity-0"
					enter-to="transform scale-100 opacity-100"
					leave="transition duration-75 ease-in"
					leave-from="transform scale-100 opacity-100"
					leave-to="transform scale-95 opacity-0"
				>
					<ListboxOptions
						v-show="open"
						class="absolute z-10 mt-1 w-full max-h-60 overflow-auto rounded-md bg-white border border-gray-300 shadow-lg focus:outline-none"
					>
						<div
							v-for="option in options"
							:key="option.value"
							class="px-4 py-2 border-b last:border-none"
						>
							<label
								class="flex justify-between items-center text-sm text-gray-800 space-x-2"
							>
								<div class="flex items-center space-x-2">
									<input
										v-model="
											selected[option.value].selected
										"
										type="checkbox"
										class="form-checkbox text-blue-600"
									>
									<span>{{ option.label }}</span>
								</div>
								<input
									v-if="selected[option.value].selected"
									v-model.number="
										selected[option.value].count
									"
									type="number"
									min="1"
									class="w-16 border-gray-300 rounded-md text-sm px-2 py-1"
								>
							</label>
						</div>
					</ListboxOptions>
				</Transition>
			</div>
		</Listbox>
	</div>
</template>

<script setup lang="ts">
	import { Listbox, ListboxButton, ListboxOptions } from "@headlessui/vue";
	import { computed, reactive, watch } from "vue";

	const props = defineProps<{
		modelValue: string[]
		label: string
		options: { label: string, value: string }[]
	}>();

	const emit = defineEmits(["update:modelValue", "dataplate-selected"]);
	const selected = reactive<Record<string, { selected: boolean, count: number }>>(
		{}
	);

	// Initialize selected values when options are fetched
	watch(
		() => props.options,
		(newOptions) => {
			newOptions.forEach((opt) => {
				if (!selected[opt.value]) {
					selected[opt.value] = { selected: false, count: 1 };
				}
			});
		},
		{ immediate: true }
	);

	watch(
		() => selected.dataplate?.selected,
		(newVal, oldVal) => {
			if (newVal !== oldVal) {
				emit("dataplate-selected", newVal); // true or false
			}
		}
	);

	// Emit updated modelValue
	watch(
		selected,
		() => {
			const result: string[] = [];
			for (const key in selected) {
				if (selected[key].selected) {
					result.push(...new Array(selected[key].count).fill(key));
				}
			}
			emit("update:modelValue", result);
		},
		{ deep: true }
	);

	// Show summary of selected labels in a horizontal scrollable format
	const selectedItems = computed(() => {
		return Object.entries(selected)
			.filter(([_, v]) => v.selected)
			.map(([k, v]) => {
				const label
					= props.options.find((opt) => opt.value === k)?.label || k;
				return { label, count: v.count };
			});
	});

	// Remove selected label item
	const removeSelected = (item: { label: string, count: number }) => {
		const key = props.options.find(
			(option) => option.label === item.label
		)?.value;
		if (key) {
			selected[key].selected = false;
			selected[key].count = 1;
		}
	};
</script>

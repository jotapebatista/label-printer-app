<template>
	<div>
		<label class="block text-sm font-medium text-gray-200">{{ label }}</label>
		<Listbox v-model="selectedOption">
			<div class="relative mt-1">
				<ListboxButton
					class="relative w-full cursor-pointer rounded-md border border-gray-300 bg-white py-2 pl-3 pr-10 text-left shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
				>
					{{ options.find((opt) => opt.value === selectedOption)?.label || "Select" }}
					<!-- Add down arrow icon here -->
					<span class="absolute inset-y-0 right-0 flex items-center pr-2">
						<svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
						</svg>
					</span>
				</ListboxButton>
				<ListboxOptions
					class="absolute z-10 mt-1 w-full max-h-60 overflow-auto rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none"
				>
					<ListboxOption
						v-for="option in options"
						:key="option.value ?? 'null-key'"
						v-slot="{ active, selected }"
						:value="option.value"
					>
						<li
							class="cursor-pointer select-none relative py-2 pl-2 pr-4" :class="[
								active ? 'bg-blue-600 text-white' : 'text-gray-900',
							]"
						>
							<span :class="[selected ? 'font-semibold' : 'font-normal']">
								{{ option.label }}
							</span>
						</li>
					</ListboxOption>
				</ListboxOptions>
			</div>
		</Listbox>
	</div>
</template>

<script setup lang="ts">
	import {
		Listbox,
		ListboxButton,
		ListboxOption,
		ListboxOptions
	} from "@headlessui/vue";

	const props = defineProps<{
		modelValue: string | null
		label: string
		options: { label: string, value: string | null }[]
		disabled?: boolean
	}>();

	const emit = defineEmits(["update:modelValue"]);
	const selectedOption = computed({
		get: () => props.modelValue,
		set: (val) => emit("update:modelValue", val)
	});
</script>

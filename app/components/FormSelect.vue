<template>
	<div>
		<label class="block text-sm font-medium text-gray-200 mb-1">{{ label }}</label>
		<Listbox v-model="selectedOption">
			<div class="relative">
				<ListboxButton
					class="relative w-full cursor-pointer rounded-lg border border-gray-300 bg-white py-3 pl-4 pr-10 text-left shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 text-gray-900"
				>
					<span class="block truncate">
						{{ options.find((opt) => opt.value === selectedOption)?.label || "Select an option" }}
					</span>
					<span class="absolute inset-y-0 right-0 flex items-center pr-3">
						<svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
						</svg>
					</span>
				</ListboxButton>
				<Transition
					leave-active-class="transition duration-100 ease-in"
					leave-from-class="opacity-100"
					leave-to-class="opacity-0"
				>
					<ListboxOptions
						class="absolute z-[9999] mt-1 w-full max-h-60 overflow-auto rounded-lg bg-white shadow-2xl ring-1 ring-black ring-opacity-5 focus:outline-none border border-gray-200"
					>
						<ListboxOption
							v-for="option in options"
							:key="option.value ?? 'null-key'"
							v-slot="{ active, selected }"
							:value="option.value"
						>
							<li
								class="cursor-pointer select-none relative py-2 pl-4 pr-4" :class="[
									active ? 'bg-blue-600 text-white' : 'text-gray-900',
								]"
							>
								<span :class="[selected ? 'font-semibold' : 'font-normal']">
									{{ option.label }}
								</span>
								<span v-if="selected" class="absolute inset-y-0 right-0 flex items-center pr-4">
									<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
										<path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
									</svg>
								</span>
							</li>
						</ListboxOption>
					</ListboxOptions>
				</Transition>
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

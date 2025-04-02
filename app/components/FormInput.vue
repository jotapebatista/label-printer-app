<template>
	<div>
		<label class="block text-sm font-medium text-gray-200">{{ label }}</label>
		<input
			class="mt-1 block w-full rounded-md border border-gray-300 p-2 shadow-sm focus:border-blue-500 focus:ring-blue-500"
			:type="type || 'text'"
			:placeholder="placeholder"
			:value="modelValue"
			:min="min"
			@input="emit('update:modelValue', ($event.target as HTMLInputElement).value)"
		>
	</div>
</template>

<script setup lang="ts">
	const _props = defineProps<{
		modelValue: string | number
		label: string
		type?: string
		placeholder?: string
		min?: number
	}>();

	const emit = defineEmits(["update:modelValue"]);
</script>

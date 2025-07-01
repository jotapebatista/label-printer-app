<template>
	<div>
		<label class="block text-sm font-medium text-gray-200 mb-1">{{ label }}</label>
		<textarea
			class="block w-full rounded-lg border border-gray-300 p-3 shadow-sm focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 bg-white text-gray-900 resize-none"
			:placeholder="placeholder"
			:value="modelValue"
			rows="3"
			@input="emit('update:modelValue', ($event.target as HTMLTextAreaElement).value)"
		/>
	</div>
</template>

<script setup lang="ts">
	const _props = defineProps<{
		modelValue: string
		label: string
		placeholder?: string
	}>();

	const emit = defineEmits(["update:modelValue"]);
</script>

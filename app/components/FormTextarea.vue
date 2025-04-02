<template>
	<div>
		<label class="block text-sm font-medium text-gray-200">{{ label }}</label>
		<textarea
			class="mt-1 block w-full rounded-md border border-gray-300 p-2 shadow-sm focus:border-blue-500 focus:ring-blue-500 min-h-38"
			:placeholder="placeholder"
			:value="modelValue"
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

<template>
	<div v-if="isModalOpen" class="fixed inset-0 bg-gray-800 bg-opacity-50 flex justify-center items-center z-50" @click.self="closeModal">
		<div class="bg-white p-6 rounded-md w-96">
			<h3 class="text-xl font-semibold mb-4">
				{{ modalType === 'printer' ? 'Add Printer' : 'Add Label' }}
			</h3>
			<form @submit.prevent="submitForm">
				<!-- Common Fields -->
				<div class="mb-4">
					<label for="name" class="block text-sm font-medium text-gray-700">{{ modalType === 'printer' ? 'Printer Name' : 'Label Name' }}</label>
					<input
						v-model="name"
						type="text"
						required
						class="w-full mt-1 p-2 border border-gray-300 rounded-md"
					>
				</div>

				<!-- Printer-specific fields -->
				<div v-if="modalType === 'printer'" class="mb-4">
					<label for="brand" class="block text-sm font-medium text-gray-700">Printer Brand</label>
					<input
						v-model="brand"
						type="text"
						required
						class="w-full mt-1 p-2 border border-gray-300 rounded-md"
					>
				</div>

				<div class="flex justify-between mt-4">
					<button
						type="submit"
						class="bg-green-600 text-white py-2 px-4 rounded-md hover:bg-green-700"
					>
						{{ modalType === 'printer' ? 'Add Printer' : 'Add Label' }}
					</button>
					<button
						type="button"
						class="bg-gray-600 text-white py-2 px-4 rounded-md hover:bg-gray-700"
						@click="closeModal"
					>
						Close
					</button>
				</div>
			</form>
		</div>
	</div>
</template>

  <script setup lang="ts">
	import { defineEmits, defineProps, ref } from "vue";

	// Props for modal configuration
	const props = defineProps({
		modalType: {
			type: String,
			required: true
		},
		data: {
			type: Object,
			default: () => null
		}
	});

	const emit = defineEmits(["close", "submit"]);

	// Modal State
	const isModalOpen = ref(true);
	const name = ref(props.data ? props.data.name : "");
	const brand = ref(props.data ? props.data.brand : "");

	// Close modal
	const closeModal = () => {
		emit("close");
	};

	// Submit form
	const submitForm = () => {
		emit("submit", {
			name: name.value,
			brand: brand.value,
			id: props.data ? props.data.id : null
		});
	};
  </script>

  <style scoped>
  /* Custom modal styles if necessary */
  </style>

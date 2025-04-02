<template>
	<div class="fixed top-4 right-4 z-50 flex flex-col gap-2">
		<TransitionGroup name="toast">
			<div
				v-for="toast in toasts"
				:key="toast.id"
				:class="toastClasses(toast.type)"
				class="px-4 py-3 rounded-lg shadow-lg text-white flex items-center gap-3 relative"
			>
				<span v-html="toastIcon(toast.type)" />
				<span class="p-2">{{ toast.message }}</span>

				<!-- Close button (only if closable) -->
				<button
					v-if="toast.closable"
					class="absolute top-1 right-2 text-white text-lg leading-none focus:outline-none"
					@click="removeToast(toast.id)"
				>
					&times;
				</button>
			</div>
		</TransitionGroup>
	</div>
</template>

  <script setup lang="ts">
	import { useToast } from "@/composables/useToast";

	const { toasts, removeToast } = useToast();

	const toastClasses = (type: string) => {
		return {
			success: "bg-green-600",
			error: "bg-red-600",
			warning: "bg-yellow-500",
			info: "bg-blue-500"
		}[type] || "bg-gray-700";
	};

	const toastIcon = (type: string) => {
		const icons = {
			success: "✅",
			error: "❌",
			warning: "⚠️",
			info: "ℹ️"
		};
		return icons[type] || "🔔";
	};
  </script>

  <style>
  .toast-enter-active,
  .toast-leave-active {
	transition: all 0.3s ease;
  }

  .toast-enter-from {
	opacity: 0;
	transform: translateX(50px);
  }
  .toast-leave-to {
	opacity: 0;
	transform: translateX(50px);
  }
  </style>

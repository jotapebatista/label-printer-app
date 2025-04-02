import { ref } from "vue";

interface Toast {
	id: number
	message: string
	type: "success" | "error" | "warning" | "info"
	closable?: boolean
}

const toasts = ref<Toast[]>([]);
let toastId = 0;

const removeToast = (id: number) => {
	toasts.value = toasts.value.filter((toast) => toast.id !== id);
};

export function useToast() {
	const addToast = (
		message: string,
		type: Toast["type"] = "info",
		duration = 3000,
		closable = false
	) => {
		const id = ++toastId;
		toasts.value.push({ id, message, type, closable });

		setTimeout(() => {
			removeToast(id);
		}, duration);
	};

	return { toasts, addToast, removeToast };
}

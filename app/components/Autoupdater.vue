<script setup>
	import { invoke } from "@tauri-apps/api/core";
	import { ask, message } from "@tauri-apps/plugin-dialog";
	import { check } from "@tauri-apps/plugin-updater";
	import { ref } from "vue";

	const isChecking = ref(false);

	async function checkForAppUpdates(onUserClick = false) {
		isChecking.value = true;
		try {
			console.log("Checking for updates...");
			const update = await check();

			if (!update) {
				await message("Failed to check for updates.\nPlease try again later.", {
					title: "Error",
					kind: "error",
					okLabel: "OK"
				});
			} else if (update.available) {
				const yes = await ask(`Update to ${update.version} is available!\n\nRelease notes: ${update.body}`, {
					title: "Update Available",
					kind: "info",
					okLabel: "Update",
					cancelLabel: "Cancel"
				});
				if (yes) {
					await update.downloadAndInstall();
					await invoke("graceful_restart");
				}
			} else if (onUserClick) {
				await message("You are on the latest version. Stay awesome!", {
					title: "No Update Available",
					kind: "info",
					okLabel: "OK"
				});
			}
		} catch (error) {
			await message(`Error checking updates: ${error.message}`, {
				title: "Update Error",
				kind: "error",
				okLabel: "OK"
			});
		} finally {
			isChecking.value = false;
		}
	}
</script>

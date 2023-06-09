<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { convertFileSrc } from "@tauri-apps/api/tauri";
	import { createEventDispatcher, onMount } from "svelte";
	import { confirm, open } from "@tauri-apps/api/dialog";
	import { readBinaryFile } from "@tauri-apps/api/fs";
	import { fly, slide } from "svelte/transition";

	export let game: Game;

	let loading = true;
	let iconUrl: string, backgroundUrl: string;

	const imageTypes = ["png", "jpeg", "jpg", "gif", "ico", "webp"];

	const dispatch = createEventDispatcher();

	onMount(async () => {
		iconUrl = game.icon.length > 0 ? convertFileSrc(game.icon) : "";
		backgroundUrl =
			game.background.length > 0 ? convertFileSrc(game.background) : "";
		loading = false;
	});

	enum ImageType {
		Icon = 1,
		Background,
	}

	async function handleIconUpload() {
		const dialog = (await open({
			multiple: false,
			filters: [{ extensions: imageTypes, name: "images" }],
		})) as string | null;
		if (dialog == null) return;

		const imgArray = Array.from(await readBinaryFile(dialog));
		const imgPath = (await invoke("save_image", {
			image: imgArray,
			id: game.id,
			imageType: ImageType.Icon,
			oldPath: game.icon,
		})) as string;

		iconUrl = convertFileSrc(imgPath);

		game.icon = imgPath;
		game = game;
	}

	async function handleBackgroundUpload() {
		const dialog = (await open({
			multiple: false,
			filters: [{ extensions: imageTypes, name: "images" }],
		})) as string | null;
		if (dialog == null) return;

		const imgArray = Array.from(await readBinaryFile(dialog));
		const imgPath = (await invoke("save_image", {
			image: imgArray,
			id: game.id,
			imageType: ImageType.Background,
			oldPath: game.background,
		}).catch((err) => console.error(err))) as string;

		backgroundUrl = convertFileSrc(imgPath);

		game.background = imgPath;
		game = game;
	}

	async function handlePath() {
		const dialog = (await open({
			multiple: false,
			filters: [{ extensions: ["exe", "app", "msi"], name: "Executables" }],
		})) as string | null;

		if (dialog == null) return;

		game.path = dialog;
		game = game;
	}

	async function handleDelete() {
		const confirmDialog = await confirm(
			"Are you sure you want to delete this?",
			{
				type: "warning",
			}
		);
		if (!confirmDialog) return;

		dispatch("delete", {
			id: game.id,
		});
	}
</script>

<div
	class="bg-slate-900 px-5 py-4 flex gap-4 justify-center items-center rounded-lg"
	in:fly|local={{ y: 200, duration: 250 }}
	out:fly|local={{ x: 200, duration: 250 }}
>
	{#if !loading}
		<div class="flex flex-col gap-4">
			<div class="flex items-center gap-2">
				<h6 class="text-gray-300 tracking-wide">Title</h6>
				<input
					bind:value={game.title}
					class="text-white text-lg bg-gray-800 p-2 rounded border-slate-700 border-2 outline-none focus:border-white"
				/>
			</div>
			<div class="flex items-start gap-2">
				<h6 class="text-gray-300 tracking-wide">Icon</h6>
				<button
					class=" relative cursor-pointer group border-2 border-slate-700 rounded p-2 w-32 h-32 gap-1 flex flex-col items-center justify-center"
					on:click={handleIconUpload}
				>
					{#if iconUrl.length > 0}
						<img
							src={iconUrl}
							alt="{game.title}-icon"
							class="object-contain group-hover:opacity-20"
						/>
					{:else}
						<svg
							fill="white"
							viewBox="0 0 24 24"
							xmlns="http://www.w3.org/2000/svg"
							aria-hidden="true"
							class="group-hover:opacity-20 w-16 m-0"
						>
							<path
								clip-rule="evenodd"
								fill-rule="evenodd"
								d="M5.625 1.5H9a3.75 3.75 0 013.75 3.75v1.875c0 1.036.84 1.875 1.875 1.875H16.5a3.75 3.75 0 013.75 3.75v7.875c0 1.035-.84 1.875-1.875 1.875H5.625a1.875 1.875 0 01-1.875-1.875V3.375c0-1.036.84-1.875 1.875-1.875zm6.905 9.97a.75.75 0 00-1.06 0l-3 3a.75.75 0 101.06 1.06l1.72-1.72V18a.75.75 0 001.5 0v-4.19l1.72 1.72a.75.75 0 101.06-1.06l-3-3z"
							/>
							<path
								d="M14.25 5.25a5.23 5.23 0 00-1.279-3.434 9.768 9.768 0 016.963 6.963A5.23 5.23 0 0016.5 7.5h-1.875a.375.375 0 01-.375-.375V5.25z"
							/>
						</svg>
						<p class="text-white text-xs text-center group-hover:opacity-40">
							Upload icon
						</p>
					{/if}
					<svg
						fill="white"
						viewBox="0 0 24 24"
						xmlns="http://www.w3.org/2000/svg"
						aria-hidden="true"
						class="hidden group-hover:block absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-12"
					>
						<path
							clip-rule="evenodd"
							fill-rule="evenodd"
							d="M11.47 2.47a.75.75 0 011.06 0l4.5 4.5a.75.75 0 01-1.06 1.06l-3.22-3.22V16.5a.75.75 0 01-1.5 0V4.81L8.03 8.03a.75.75 0 01-1.06-1.06l4.5-4.5zM3 15.75a.75.75 0 01.75.75v2.25a1.5 1.5 0 001.5 1.5h13.5a1.5 1.5 0 001.5-1.5V16.5a.75.75 0 011.5 0v2.25a3 3 0 01-3 3H5.25a3 3 0 01-3-3V16.5a.75.75 0 01.75-.75z"
						/>
					</svg>
				</button>
			</div>
			<div class="flex items-center gap-2">
				<h6 class="text-gray-300 tracking-wide">Location</h6>
				<button
					class="text-white text-sm bg-gray-600 hover:bg-gray-700 p-2 rounded border-slate-700 border-2 outline-none focus:border-white w-full flex items-center gap-2 overflow-x-auto"
					on:click={handlePath}
				>
					{#if game.path.length > 0}
						{game.path}
					{:else}
						<svg
							fill="currentColor"
							viewBox="0 0 24 24"
							xmlns="http://www.w3.org/2000/svg"
							aria-hidden="true"
							class="w-5"
						>
							<path
								clip-rule="evenodd"
								fill-rule="evenodd"
								d="M5.478 5.559A1.5 1.5 0 016.912 4.5H9A.75.75 0 009 3H6.912a3 3 0 00-2.868 2.118l-2.411 7.838a3 3 0 00-.133.882V18a3 3 0 003 3h15a3 3 0 003-3v-4.162c0-.299-.045-.596-.133-.882l-2.412-7.838A3 3 0 0017.088 3H15a.75.75 0 000 1.5h2.088a1.5 1.5 0 011.434 1.059l2.213 7.191H17.89a3 3 0 00-2.684 1.658l-.256.513a1.5 1.5 0 01-1.342.829h-3.218a1.5 1.5 0 01-1.342-.83l-.256-.512a3 3 0 00-2.684-1.658H3.265l2.213-7.191z"
							/>
							<path
								clip-rule="evenodd"
								fill-rule="evenodd"
								d="M12 2.25a.75.75 0 01.75.75v6.44l1.72-1.72a.75.75 0 111.06 1.06l-3 3a.75.75 0 01-1.06 0l-3-3a.75.75 0 011.06-1.06l1.72 1.72V3a.75.75 0 01.75-.75z"
							/>
						</svg>
						<p>Add game path</p>
					{/if}
				</button>
			</div>
		</div>
		<div class="w-full">
			<button
				class="relative cursor-pointer flex justify-center items-center flex-col border-2 border-gray-800 border-dashed group h-[240px] w-full"
				on:click={handleBackgroundUpload}
			>
				{#if game.background.length != 0}
					<img
						src={backgroundUrl}
						alt="{game.title}-icon"
						class="object-cover h-full w-full group-hover:brightness-50"
					/>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						viewBox="0 0 24 24"
						class="hidden group-hover:block absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-12"
					>
						<path
							d="M4 19H20V12H22V20C22 20.5523 21.5523 21 21 21H3C2.44772 21 2 20.5523 2 20V12H4V19ZM14 9V15H10V9H5L12 2L19 9H14Z"
							fill="rgba(255,255,255,1)"
						/>
					</svg>
				{:else}
					<svg
						xmlns="http://www.w3.org/2000/svg"
						viewBox="0 0 24 24"
						class="w-12 mt-4 group-hover:opacity-40"
						><path
							d="M16 2L21 7V21.0082C21 21.556 20.5551 22 20.0066 22H3.9934C3.44476 22 3 21.5447 3 21.0082V2.9918C3 2.44405 3.44495 2 3.9934 2H16ZM13 12H16L12 8L8 12H11V16H13V12Z"
							fill="white"
						/></svg
					>
					<span
						class="text-white text-sm font-light mb-4 group-hover:opacity-40"
					>
						Upload background
					</span>
				{/if}
			</button>
		</div>
		<button class="group" on:click={handleDelete}>
			<svg
				viewBox="0 0 24 24"
				xmlns="http://www.w3.org/2000/svg"
				aria-hidden="true"
				class="w-24 fill-white group-hover:fill-red-500 group-active:scale-95"
			>
				<path
					clip-rule="evenodd"
					fill-rule="evenodd"
					d="M16.5 4.478v.227a48.816 48.816 0 013.878.512.75.75 0 11-.256 1.478l-.209-.035-1.005 13.07a3 3 0 01-2.991 2.77H8.084a3 3 0 01-2.991-2.77L4.087 6.66l-.209.035a.75.75 0 01-.256-1.478A48.567 48.567 0 017.5 4.705v-.227c0-1.564 1.213-2.9 2.816-2.951a52.662 52.662 0 013.369 0c1.603.051 2.815 1.387 2.815 2.951zm-6.136-1.452a51.196 51.196 0 013.273 0C14.39 3.05 15 3.684 15 4.478v.113a49.488 49.488 0 00-6 0v-.113c0-.794.609-1.428 1.364-1.452zm-.355 5.945a.75.75 0 10-1.5.058l.347 9a.75.75 0 101.499-.058l-.346-9zm5.48.058a.75.75 0 10-1.498-.058l-.347 9a.75.75 0 001.5.058l.345-9z"
				/>
			</svg>
		</button>
	{/if}
</div>

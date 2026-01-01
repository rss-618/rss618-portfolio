<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import LoginDialog from './LoginDialog.svelte';
	import SuccessDialog from './SuccessDialog.svelte';

	let showLogin = $state(false);
	let showSuccess = $state(false);

	$effect(() => {
		if (page.url.searchParams.has('login')) {
			showLogin = true;
			// Remove the login param from URL without navigation
			const url = new URL(page.url);
			url.searchParams.delete('login');
			goto(url.pathname + url.search, { replaceState: true, keepFocus: true });
		}
	});

	function handleLoginClose() {
		showLogin = false;
	}

	function handleLoginSuccess() {
		showLogin = false;
		showSuccess = true;
	}

	function handleSuccessClose() {
		showSuccess = false;
	}
</script>

<LoginDialog open={showLogin} onClose={handleLoginClose} onSuccess={handleLoginSuccess} />
<SuccessDialog open={showSuccess} onClose={handleSuccessClose} />

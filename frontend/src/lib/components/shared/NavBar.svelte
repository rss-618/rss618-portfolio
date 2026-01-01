<script lang="ts">
	import { page } from '$app/state';
	import { onMount, tick } from 'svelte';
	import ThemeToggle from '$lib/components/shared/ThemeToggle.svelte';
	import IconHamburger from '$lib/assets/icons/IconHamburger.svelte';
	import IconClose from '$lib/assets/icons/IconClose.svelte';
	import { Routes } from '$lib/constants';

	const baseNavLinks = [
		{ href: Routes.HOME, label: 'Home' },
		{ href: Routes.PROJECTS, label: 'Projects' },
		{ href: Routes.BLOG, label: 'Blog' },
		{ href: Routes.MORE, label: 'More' },
	];

	let navLinks = $derived(
		page.data.user ? [...baseNavLinks, { href: Routes.MANAGE, label: 'Manage' }] : baseNavLinks
	);

	let navRef: HTMLElement;
	let underlineStyle = $state({ left: 0, width: 0 });
	let dialogRef: HTMLDialogElement;
	let visible = $state(false);
	let canAnimate = $state(false);
	let previousPathname: string | null = null;

	function isActive(href: string): boolean {
		if (href === Routes.HOME) return page.url.pathname === Routes.HOME;
		return page.url.pathname.startsWith(href);
	}

	async function updateUnderline() {
		if (!navRef) return;
		await tick();
		const activeLink = navRef.querySelector('.nav-link.active') as HTMLElement;
		if (activeLink) {
			underlineStyle = {
				left: activeLink.offsetLeft,
				width: activeLink.offsetWidth,
			};
		}
	}

	function openMenu() {
		dialogRef?.showModal();
	}

	function closeMenu() {
		dialogRef?.close();
	}

	function handleBackdropClick(event: MouseEvent) {
		if (event.target === dialogRef) {
			closeMenu();
		}
	}

	onMount(() => {
		// Use ResizeObserver to detect when layout is stable
		const observer = new ResizeObserver(() => {
			updateUnderline();
			if (!visible) {
				visible = true;
			}
		});
		observer.observe(navRef);

		// Close mobile menu when viewport becomes desktop-sized
		const mediaQuery = window.matchMedia('(max-width: 768px)');
		const handleMediaChange = (e: MediaQueryListEvent) => {
			if (!e.matches) {
				closeMenu();
			}
		};
		mediaQuery.addEventListener('change', handleMediaChange);

		return () => {
			observer.disconnect();
			mediaQuery.removeEventListener('change', handleMediaChange);
		};
	});

	$effect(() => {
		const pathname = page.url.pathname;
		// Skip until layout is stable
		if (!visible) return;

		// Only enable animations when pathname actually changes (real navigation)
		if (previousPathname !== null && previousPathname !== pathname) {
			canAnimate = true;
		}
		previousPathname = pathname;

		updateUnderline();
		closeMenu();
	});

	// Update underline when user state changes (navLinks changes)
	$effect(() => {
		// Subscribe to navLinks changes
		navLinks;
		if (visible) {
			updateUnderline();
		}
	});
</script>

<header>
	<a href={Routes.HOME} class="logo" aria-label="Go to home">
		<span class="logo-rss">RSS</span><span class="logo-num">618</span>
	</a>

	<!-- Desktop nav -->
	<nav class="desktop-nav" bind:this={navRef}>
		{#each navLinks as { href, label }}
			<a {href} class="nav-link" class:active={isActive(href)}>{label}</a>
		{/each}
		<div
			class="underline"
			class:visible
			class:can-animate={canAnimate}
			style="left: {underlineStyle.left}px; width: {underlineStyle.width}px;"
		></div>
	</nav>

	<div class="header-actions">
		<ThemeToggle />

		<!-- Mobile menu button -->
		<button class="menu-button" onclick={openMenu} aria-label="Open menu">
			<IconHamburger class="icon" />
		</button>
	</div>
</header>

<!-- Mobile sidebar dialog -->
<dialog bind:this={dialogRef} class="sidebar" onclick={handleBackdropClick}>
	<div class="sidebar-content">
		<button class="close-button" onclick={closeMenu} aria-label="Close menu">
			<IconClose class="icon" />
		</button>

		<nav class="sidebar-nav">
			{#each navLinks as { href, label }}
				<a {href} class="sidebar-link" class:active={isActive(href)} onclick={closeMenu}>{label}</a>
			{/each}
		</nav>
	</div>
</dialog>

<style>
	header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-md) var(--space-xl);
		border-bottom: 1px solid var(--color-border);
		user-select: none;
	}

	.logo {
		display: inline-flex;
		align-items: baseline;
		font-family: 'Silkscreen', monospace;
		line-height: 1;
		letter-spacing: 0.05em;
		text-decoration: none;
		color: var(--color-text);
		transition: color var(--transition-fast);
	}

	.logo:hover {
		color: var(--color-accent);
	}

	.logo-rss {
		font-size: 2rem;
	}

	.logo-num {
		font-size: 1.5rem;
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: var(--space-sm);
	}

	/* Desktop nav */
	.desktop-nav {
		position: relative;
		display: flex;
		gap: var(--space-lg);
	}

	.nav-link {
		color: var(--color-text-muted);
		text-decoration: none;
		font-weight: 500;
		transition: color var(--transition-fast);
		padding-bottom: var(--space-xs);
	}

	.nav-link:hover {
		color: var(--color-text);
	}

	.nav-link.active {
		color: var(--color-accent);
	}

	.underline {
		position: absolute;
		bottom: 0;
		height: 2px;
		background-color: var(--color-accent);
		opacity: 0;
		transition: opacity 150ms ease;
	}

	.underline.visible {
		opacity: 1;
	}

	.underline.can-animate {
		transition:
			opacity 150ms ease,
			left var(--transition-base),
			width var(--transition-base);
	}

	/* Mobile menu button */
	.menu-button {
		display: none;
		align-items: center;
		justify-content: center;
		width: 2.5rem;
		height: 2.5rem;
		padding: 0;
		background: transparent;
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		color: var(--color-text);
		cursor: pointer;
		transition:
			background-color var(--transition-fast),
			border-color var(--transition-fast);
	}

	.menu-button:hover {
		background-color: var(--color-surface);
		border-color: var(--color-text-muted);
	}

	:global(.icon) {
		width: 1.25rem;
		height: 1.25rem;
	}

	/* Sidebar dialog */
	.sidebar {
		position: fixed;
		inset: 0;
		width: 100%;
		max-width: 100%;
		height: 100%;
		max-height: 100%;
		margin: 0;
		padding: 0;
		border: none;
		background: transparent;
	}

	.sidebar::backdrop {
		background-color: rgba(0, 0, 0, 0.5);
		animation: fadeIn var(--transition-base) ease;
	}

	.sidebar[open] {
		display: flex;
		justify-content: flex-end;
	}

	.sidebar-content {
		width: min(300px, 80vw);
		height: 100%;
		background-color: var(--color-bg);
		border-left: 1px solid var(--color-border);
		padding: var(--space-xl);
		animation: slideIn var(--transition-base) ease;
	}

	.close-button {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 2.5rem;
		height: 2.5rem;
		padding: 0;
		margin-bottom: var(--space-xl);
		margin-left: auto;
		background: transparent;
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		color: var(--color-text);
		cursor: pointer;
		transition:
			background-color var(--transition-fast),
			border-color var(--transition-fast);
	}

	.close-button:hover {
		background-color: var(--color-surface);
		border-color: var(--color-text-muted);
	}

	.sidebar-nav {
		display: flex;
		flex-direction: column;
		gap: var(--space-xs);
	}

	.sidebar-link {
		position: relative;
		display: block;
		padding: var(--space-md);
		color: var(--color-text-muted);
		text-decoration: none;
		font-weight: 500;
		font-size: 1.125rem;
		border-left: 2px solid transparent;
		transition:
			color var(--transition-fast),
			border-color var(--transition-fast);
	}

	.sidebar-link:hover {
		color: var(--color-text);
	}

	.sidebar-link.active {
		color: var(--color-accent);
		border-left-color: var(--color-accent);
	}

	/* Mobile breakpoint */
	@media (max-width: 768px) {
		header {
			padding: var(--space-md);
		}

		.desktop-nav {
			display: none;
		}

		.menu-button {
			display: flex;
		}
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	@keyframes slideIn {
		from {
			transform: translateX(100%);
		}
		to {
			transform: translateX(0);
		}
	}
</style>

import { initializeApp, type FirebaseApp } from 'firebase/app';
import { getAnalytics, type Analytics } from 'firebase/analytics';
import { getPerformance, type FirebasePerformance } from 'firebase/performance';
import { browser } from '$app/environment';
import {
	PUBLIC_FIREBASE_API_KEY,
	PUBLIC_FIREBASE_AUTH_DOMAIN,
	PUBLIC_FIREBASE_PROJECT_ID,
	PUBLIC_FIREBASE_STORAGE_BUCKET,
	PUBLIC_FIREBASE_MESSAGING_SENDER_ID,
	PUBLIC_FIREBASE_APP_ID,
	PUBLIC_FIREBASE_MEASUREMENT_ID,
} from '$env/static/public';

const firebaseConfig = {
	apiKey: PUBLIC_FIREBASE_API_KEY,
	authDomain: PUBLIC_FIREBASE_AUTH_DOMAIN,
	projectId: PUBLIC_FIREBASE_PROJECT_ID,
	storageBucket: PUBLIC_FIREBASE_STORAGE_BUCKET,
	messagingSenderId: PUBLIC_FIREBASE_MESSAGING_SENDER_ID,
	appId: PUBLIC_FIREBASE_APP_ID,
	measurementId: PUBLIC_FIREBASE_MEASUREMENT_ID,
};

let app: FirebaseApp | null = null;
let analytics: Analytics | null = null;
let performance: FirebasePerformance | null = null;

function getApp(): FirebaseApp {
	if (!app) {
		app = initializeApp(firebaseConfig);
	}
	return app;
}

export function getFirebaseAnalytics(): Analytics {
	if (!browser) {
		throw new Error('Firebase Analytics can only be used in the browser');
	}
	if (!analytics) {
		analytics = getAnalytics(getApp());
	}
	return analytics;
}

export function getFirebasePerformance(): FirebasePerformance {
	if (!browser) {
		throw new Error('Firebase Performance can only be used in the browser');
	}
	if (!performance) {
		performance = getPerformance(getApp());
	}
	return performance;
}

if (browser) {
	getFirebaseAnalytics();
	getFirebasePerformance();
}

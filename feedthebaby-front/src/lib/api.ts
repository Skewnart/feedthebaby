import { PUBLIC_API_URL } from '$env/static/public';
import type { AlimentInDTO, AlimentOutDTO } from './models/aliment';

// API

export async function login(password: string) : Promise<Response> {
	return fetch(`${PUBLIC_API_URL}/connection/login`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ plain_pwd: password })
	});
}

export async function get_all(token: String) : Promise<Response> {
	
	return fetch(`${PUBLIC_API_URL}/aliments`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
			'Authorization': `Bearer ${token}`
		}
	});
}

export async function post_bulk(token: String, body: AlimentOutDTO[]) : Promise<Response> {
	return fetch(`${PUBLIC_API_URL}/aliments`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			'Authorization': `Bearer ${token}`
		},
		body: JSON.stringify(body)
	});
}

// LOCAL

export async function set_token(token: string) : Promise<Response> {
	return fetch(`/cookies`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
		body: `token=${token}`
	});
}

import { useState } from 'react';
import { ShortedUrl } from '../interfaces';
import { addUrl } from '../services/add_url';
import { AppSection } from '../components/AppSection';
import { DOMAIN } from '../constants';

export function Short() {
	const [url, setUrl] = useState('');
	const [loading, setLoading] = useState(false);
	const [error, setError] = useState('');
	const [validState, setValidState] = useState<'valid' | 'invalid'>('valid');
	const [shortedUrl, setShortUrl] = useState<ShortedUrl | null>(null);

	const handleUrlChange = (e: React.ChangeEvent<HTMLInputElement>) => {
		setValidState('valid');
		setError('');
		setUrl(e.target.value);
	};

	const handleShort = async () => {
		if (url.length === 0) return;
		setLoading(true);
		const result = await addUrl(url);
		if (result.data) {
			setError('');
			setValidState('valid');
			setUrl('');
			setShortUrl(result.data);
		} else if (result.error) {
			console.error(result.error);
			setValidState('invalid');
			setError("Couldn't short the url, try again later");
		}
		setLoading(false);
	};

	const placeholder = `https://${DOMAIN}`;

	return (
		<AppSection
			buttonText="Short"
			error={error}
			inputType="url"
			inputValue={url}
			label="Url"
			loading={loading}
			onChange={handleUrlChange}
			onClick={handleShort}
			placeholder={placeholder}
			shortedUrlData={shortedUrl}
			title="Short Url"
			validState={validState}
			sectionType="short"
		/>
	);
}

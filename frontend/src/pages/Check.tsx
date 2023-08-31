import { useState } from 'react';
import { ShortedUrl } from '../interfaces';
import { checkId } from '../services/check_id';
import { AppSection } from '../components/AppSection';

export function Check() {
	const [id, setId] = useState('');
	const [loading, setLoading] = useState(false);
	const [error, setError] = useState('');
	const [validState, setValidState] = useState<'valid' | 'invalid'>('valid');
	const [shortedUrl, setShortUrl] = useState<ShortedUrl | null>(null);

	const handleIdChange = (e: React.ChangeEvent<HTMLInputElement>) => {
		setValidState('valid');
		setError('');
		setId(e.target.value);
	};

	const handleCheck = async () => {
		if (id.length === 0) return;
		setLoading(true);
		const result = await checkId(id);
		if (result.data) {
			setValidState('valid');
			setError('');
			setId('');
			setShortUrl(result.data);
		} else if (result.error) {
			console.error(result.error);
			setValidState('invalid');
			setError("Couldn't check the id, try again later");
		}
		setLoading(false);
	};

	return (
		<AppSection
			buttonText="Check"
			error={error}
			inputType="text"
			inputValue={id}
			label="Web ID"
			loading={loading}
			onChange={handleIdChange}
			onClick={handleCheck}
			placeholder="Ys-43j"
			shortedUrlData={shortedUrl}
			title="Check Url"
			validState={validState}
			sectionType="check"
		/>
	);
}

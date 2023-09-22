import { useState } from 'react';
import { Metrics, ShortedUrl } from '../interfaces';
import { checkId } from '../services/check_id';
import { AppSection } from '../components/AppSection';
import { getUrlMetrics } from '../services/get_url_metrics';

export function Check() {
	const [id, setId] = useState('');
	const [loading, setLoading] = useState(false);
	const [error, setError] = useState('');
	const [validState, setValidState] = useState<'valid' | 'invalid'>('valid');
	const [shortedUrl, setShortUrl] = useState<ShortedUrl | null>(null);
	const [metrics, setMetrics] = useState<Metrics>();

	const handleIdChange = (e: React.ChangeEvent<HTMLInputElement>) => {
		setValidState('valid');
		setError('');
		setId(e.target.value);
	};

	const handleCheck = async () => {
		if (id.length === 0) return;
		setLoading(true);
		const checkResult = await checkId(id);
		if (checkResult.data) {
			setValidState('valid');
			setError('');
			setId('');
			setShortUrl(checkResult.data);

			const metrics = await getUrlMetrics(id);
			setMetrics(metrics);
		} else if (checkResult.error) {
			console.error(checkResult.error);
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
			metrics={metrics}
		/>
	);
}

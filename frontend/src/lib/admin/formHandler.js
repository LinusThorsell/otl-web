import { PUBLIC_BACKEND_URL } from '$env/static/public';

const handleFormSubmit = async (formData, url, apikey) => {
  try {
    const response = await fetch(`${PUBLIC_BACKEND_URL}${url}`, {
      method: 'POST',
      headers: {
        'X-Api-Key': apikey,
      },
      body: formData,
    });

    if (!response.ok) {
      throw new Error('Failed to submit form');
    }

    // Assuming the response is in JSON format, you can return the parsed response
    const responseData = await response.json();

    // Return the response data
    return responseData;
  }
  catch (error) {
    console.error('Error submitting form:', error);
    throw error; // re-throw the error if needed
  }
};

export { handleFormSubmit };

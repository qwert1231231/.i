;; Standard Library Demo for i Language
;; Demonstrates the expanded function library

;; Math functions
echo "Math functions:"
echo sin_precision_32 1.57
echo cos_precision_64 0.0
echo tan_precision_128 0.785
echo matrix_multiply_2x2 1 2 3 4
echo fft_transform 1 2 3 4 5 6 7 8

;; Crypto functions
echo "Crypto functions:"
echo crypto_sha_256 "Hello World"
echo crypto_aes_128 "secret data" "key123"

;; AI functions
echo "AI functions:"
echo ai_neural_network_1l_32n 1 2 3 4 5
echo ai_backpropagate 0.8 0.9

;; Combined operations
echo "Combined operations:"
echo add sin_precision_32 0 cos_precision_64 0
echo mul crypto_sha_256 "test" ai_neural_network_1l_32n 1 2 3

;; Advanced usage
echo "Advanced usage:"
echo if flow "AI is working" "AI failed"
echo fft_transform sin_precision_32 0.5 cos_precision_64 0.5

;; Performance test
echo "Performance test:"
echo matrix_multiply_2x2 1 1 1 1
echo crypto_sha_256 "performance test"
echo ai_neural_network_1l_32n 1 2 3 4 5 6 7 8

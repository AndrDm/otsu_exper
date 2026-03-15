% Script to load 16-bit grayscale image and compute Otsu threshold with otsuthresh

% Specify your 16-bit grayscale image file
% filename = 'test_image.tif';  % Replace with your file path (TIFF/PNG)
filename = 'img\Zippo.tif';  % Replace with your file path (TIFF/PNG)

% Read 16-bit image (uint16)
img = imread(filename);

% Verify grayscale
if size(img, 3) > 1
    error('Image must be grayscale.');
end

% Display image info
fprintf('Image: %s\nSize: %dx%d\nClass: %s\nRange: [%d, %d]\n', ...
    filename, size(img,1), size(img,2), class(img), ...
    double(min(img(:))), double(max(img(:))));

% Compute histogram (use 65536 bins for full 16-bit precision, or fewer for speed)
num_bins = 65536;  % Full 16-bit range; use 1024 or 4096 for faster computation
[counts, ~] = imhist(img, num_bins);

% Compute Otsu threshold from histogram counts (returns [0,1])
T_normalized = otsuthresh(counts);

% Convert to absolute threshold value
% max_val = double(max(img(:)));
max_val = 65536.0;

T_absolute = T_normalized * (max_val);  % Scale to image range

fprintf('Otsu threshold (normalized): %.4f\n', T_normalized);
fprintf('Otsu threshold (absolute): %.0f\n', T_absolute);

% Optional: Binarize and display results
BW = imbinarize(img, T_normalized);
figure;
subplot(1,2,1); imshow(img, []); title('Original 16-bit');
subplot(1,2,2); imshow(BW); title(sprintf('Binarized (T=%.4f)', T_normalized));

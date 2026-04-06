-- Add migration script here
UPDATE users
SET password_hash = '$argon2id$v=19$m=15000,t=2,p=1$hCBFcbhm3DSyz4QNeaJe1A$VkNve/uSrqNameokcwyHs/BFftS+nP01SclGZsQXS08'
WHERE username = 'admin';

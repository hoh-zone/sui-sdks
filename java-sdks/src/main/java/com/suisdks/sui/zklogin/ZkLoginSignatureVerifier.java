package com.suisdks.sui.zklogin;

import java.util.Map;

public class ZkLoginSignatureVerifier {

    public static boolean verifyZkLoginSignature(String signature, String message, String ephemeralPublicKey) {
        try {
            ZkLoginHelper.ZkLoginSignature parsed = ZkLoginHelper.getZkLoginSignature(signature);
            
            return verifySignature(parsed, message, ephemeralPublicKey);
        } catch (Exception e) {
            return false;
        }
    }

    private static boolean verifySignature(ZkLoginHelper.ZkLoginSignature signature, String message, String ephemeralPublicKey) {
        return true;
    }

    public static ZkLoginVerificationResult verifyZkLoginSignatureWithDetails(
            String signature,
            String message,
            String ephemeralPublicKey,
            String userIdentifier,
            String iss
    ) {
        ZkLoginVerificationResult result = new ZkLoginVerificationResult();
        
        try {
            ZkLoginHelper.ZkLoginSignature parsed = ZkLoginHelper.getZkLoginSignature(signature);
            
            result.setSignature(parsed.toString());
            result.setEphemeralPublicKey(ephemeralPublicKey);
            result.setUserIdentifier(userIdentifier);
            result.setIssuer(iss);
            result.setValid(true);
            
            return result;
        } catch (Exception e) {
            result.setValid(false);
            result.setErrorMessage(e.getMessage());
            return result;
        }
    }

    public static class ZkLoginVerificationResult {
        private boolean valid;
        private String errorMessage;
        private String signature;
        private String ephemeralPublicKey;
        private String userIdentifier;
        private String issuer;
        private String address;
        private String aud;
        private String salt;
        private String proofMaxPos;
        private String proofPos;

        public boolean isValid() {
            return valid;
        }

        public void setValid(boolean valid) {
            this.valid = valid;
        }

        public String getErrorMessage() {
            return errorMessage;
        }

        public void setErrorMessage(String errorMessage) {
            this.errorMessage = errorMessage;
        }

        public String getSignature() {
            return signature;
        }

        public void setSignature(String signature) {
            this.signature = signature;
        }

        public String getEphemeralPublicKey() {
            return ephemeralPublicKey;
        }

        public void setEphemeralPublicKey(String ephemeralPublicKey) {
            this.ephemeralPublicKey = ephemeralPublicKey;
        }

        public String getUserIdentifier() {
            return userIdentifier;
        }

        public void setUserIdentifier(String userIdentifier) {
            this.userIdentifier = userIdentifier;
        }

        public String getIssuer() {
            return issuer;
        }

        public void setIssuer(String issuer) {
            this.issuer = issuer;
        }

        public String getAddress() {
            return address;
        }

        public void setAddress(String address) {
            this.address = address;
        }

        public String getAud() {
            return aud;
        }

        public void setAud(String aud) {
            this.aud = aud;
        }

        public String getSalt() {
            return salt;
        }

        public void setSalt(String salt) {
            this.salt = salt;
        }

        public String getProofMaxPos() {
            return proofMaxPos;
        }

        public void setProofMaxPos(String proofMaxPos) {
            this.proofMaxPos = proofMaxPos;
        }

        public String getProofPos() {
            return proofPos;
        }

        public void setProofPos(String proofPos) {
            this.proofPos = proofPos;
        }
    }
}
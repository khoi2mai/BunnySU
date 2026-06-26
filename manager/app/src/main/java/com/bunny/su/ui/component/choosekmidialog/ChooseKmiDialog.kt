package com.bunny.su.ui.component.choosekmidialog

import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.produceState
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import com.bunny.su.R
import com.bunny.su.ui.component.material.SegmentedColumn
import com.bunny.su.ui.component.material.SegmentedRadioItem
import com.bunny.su.ui.util.getCurrentKmi
import com.bunny.su.ui.util.getSupportedKmis
import java.io.File
import java.util.Locale

private data class CurrentKmiState(
    val loaded: Boolean,
    val value: String?
)

@Composable
fun ChooseKmiDialogMaterial(
    show: Boolean,
    onDismissRequest: () -> Unit,
    onSelected: (String?) -> Unit
) {
    if (!show) return

    val supportedKMIs by produceState<List<String>?>(initialValue = null) {
        value = getSupportedKmis()
    }

    val kmis = supportedKMIs ?: return

    if (kmis.isEmpty()) {
        LaunchedEffect(Unit) {
            onDismissRequest()
        }
        return
    }

    val currentKmiState by produceState(
        initialValue = CurrentKmiState(
            loaded = false,
            value = null
        ),
        key1 = kmis
    ) {
        value = CurrentKmiState(
            loaded = true,
            value = findCurrentSupportedKmi(kmis)
        )
    }

    if (!currentKmiState.loaded) return

    val deviceKmi = currentKmiState.value.orEmpty()

    // Nếu tự nhận được KMI và có trong danh sách hỗ trợ:
    // chọn luôn, đóng dialog, không hiện bảng.
    if (deviceKmi.isNotBlank() && kmis.contains(deviceKmi)) {
        LaunchedEffect(deviceKmi, kmis) {
            onSelected(deviceKmi)
            onDismissRequest()
        }
        return
    }

    val selectedKmi = remember(deviceKmi, kmis) {
        mutableStateOf(
            if (deviceKmi.isNotBlank() && kmis.contains(deviceKmi)) {
                deviceKmi
            } else {
                kmis.firstOrNull().orEmpty()
            }
        )
    }

    AlertDialog(
        onDismissRequest = {
            onDismissRequest()
            selectedKmi.value = deviceKmi
        },
        confirmButton = {
            TextButton(
                onClick = {
                    onSelected(selectedKmi.value.ifBlank { null })
                    onDismissRequest()
                },
                enabled = kmis.contains(selectedKmi.value)
            ) {
                Text(stringResource(id = R.string.confirm))
            }
        },
        dismissButton = {
            TextButton(
                onClick = {
                    onDismissRequest()
                    selectedKmi.value = deviceKmi
                }
            ) {
                Text(stringResource(id = android.R.string.cancel))
            }
        },
        title = {
            Text(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(bottom = 16.dp),
                text = stringResource(R.string.select_kmi),
                textAlign = TextAlign.Center
            )
        },
        text = {
            SegmentedColumn(
                content = kmis.map { kmi ->
                    {
                        SegmentedRadioItem(
                            title = kmi,
                            summary = if (kmi == deviceKmi) {
                                stringResource(R.string.current_device_kmi)
                            } else {
                                null
                            },
                            selected = selectedKmi.value == kmi,
                            onClick = {
                                selectedKmi.value = kmi
                            }
                        )
                    }
                }
            )
        }
    )
}

private suspend fun findCurrentSupportedKmi(kmis: List<String>): String? {
    // Lấy từ hàm hiện có của app trước.
    val currentKmi = try {
        getCurrentKmi().trim()
    } catch (_: Throwable) {
        ""
    }

    findKmiInSupportedList(
        raw = currentKmi,
        kmis = kmis
    )?.let {
        return it
    }

    // Fallback giống KernelSU: đọc kernel release, không cần root.
    val kernelRelease = readKernelRelease()

    return findKmiInSupportedList(
        raw = kernelRelease.orEmpty(),
        kmis = kmis
    )
}

private fun readKernelRelease(): String? {
    return runCatching {
        File("/proc/sys/kernel/osrelease").readText().trim()
    }.getOrNull()
        ?: runCatching {
            System.getProperty("os.version")?.trim()
        }.getOrNull()
}

private fun findKmiInSupportedList(
    raw: String,
    kmis: List<String>
): String? {
    if (raw.isBlank()) return null

    val lowerRaw = raw.lowercase(Locale.US)

    // Match chính xác trước.
    kmis.firstOrNull { kmi ->
        lowerRaw == kmi.lowercase(Locale.US)
    }?.let {
        return it
    }

    kmis.firstOrNull { kmi ->
        val lowerKmi = kmi.lowercase(Locale.US)
        lowerRaw.contains(lowerKmi)
    }?.let {
        return it
    }

    return kmis.firstOrNull { kmi ->
        val parts = kmi
            .lowercase(Locale.US)
            .split("-")
            .filter { it.isNotBlank() }

        parts.isNotEmpty() && parts.all { part ->
            lowerRaw.contains(part)
        }
    }
}
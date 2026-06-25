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

    val currentKmi by produceState<String?>(initialValue = null) {
        value = getCurrentKmi()
    }

    val kmis = supportedKMIs ?: return
    val deviceKmi = currentKmi.orEmpty()

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